//! 组合根：创建窗口 / 引擎 / 通道 / 计时器，恢复设置，接线回调与泵，
//! 进入事件循环。main.rs 只调用 run()；各领域细节在兄弟模块中。

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc;
use std::time::Duration;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::audio_engine::{AudioEngine, Command, EqSettings, PlaybackMode};
use crate::events::{self, FileEvent, UpdateEvent};
use crate::playlist::{PlaylistView, add_track, play_file_now};
use crate::pumps::{particle_33ms, pump_events, spawn_ui_watchdog};
use crate::settings::{load_settings, save_settings};
use crate::transport::{SeekState, ThemeTween};
use crate::ui_callbacks::{register_callbacks, register_playlist_window_callbacks};
use crate::version::app_version;
use crate::waveform::{WaveformResult, spawn_waveform_worker};
use crate::waveform_cache::trim_wave_cache;
use crate::windows_platform::{
    apply_system_effects, enforce_single_instance, hwnd_from_window, screen_size, set_about_open,
    set_always_on_top, set_file_events, set_playlist_open, setup_drag_drop, setup_playlist_window,
    window_rect_px,
};
use crate::{
    AboutState, EqState, MainWindow, PlaylistEntry, PlaylistState, PlaylistWindow, TransportState,
    UpdateState,
};

/// 组合根持有的全部共享状态。UI 线程独占（整体 Rc），闭包经 Weak<App>
/// 临时升级访问——与拆分前“闭包克隆各自 Rc”等价，但依赖清单集中一处。
pub struct App {
    pub ui: MainWindow,
    pub audio: Rc<AudioEngine>,
    pub playlist: Rc<RefCell<Vec<PathBuf>>>,
    pub playlist_view: Rc<RefCell<PlaylistView>>,
    pub wave_bars_model: Rc<VecModel<f32>>,
    pub waveform_cache: Rc<RefCell<HashMap<PathBuf, WaveformResult>>>,
    pub cache_order: Rc<RefCell<VecDeque<PathBuf>>>,
    pub mode_cell: Rc<Cell<PlaybackMode>>,
    pub seek_wait: Rc<RefCell<Option<SeekState>>>,
    pub bg_front: Rc<Cell<bool>>,
    pub theme_tween: Rc<ThemeTween>,
    pub file_tx: mpsc::Sender<FileEvent>,
    pub file_rx: mpsc::Receiver<FileEvent>,
    pub wave_tx: mpsc::Sender<PathBuf>,
    pub wave_rx: mpsc::Receiver<WaveformResult>,
    pub update_tx: mpsc::Sender<UpdateEvent>,
    pub update_rx: mpsc::Receiver<UpdateEvent>,
    pub mode_hide_timer: Rc<slint::Timer>,
    pub popup_hide_timer: Rc<slint::Timer>,
    /// 引擎侧均衡器参数：EqState 回调改写（RefCell——Rc<App> 共享下的
    /// 运行时可变性），do_close 时随 settings 持久化。
    pub eq: RefCell<EqSettings>,
    /// EQ 面板 10 段增益的 Slint 模型：滑条回写与设置恢复的单一数据源，
    /// app.eq（引擎侧 EqSettings）在每次改动时从本模型重建。
    pub eq_gains_model: Rc<VecModel<f32>>,
    /// 磁贴状态：弹窗是否贴靠主窗（true = 跟随主窗移动）+ 弹窗相对主窗
    /// 左上角的偏移（物理像素）+ 贴靠方位（0=未贴靠 1=主窗右侧 2=左侧
    /// 3=下方 4=上方，供光效提示选择发光边）。弹出默认贴靠；拖离阈值
    /// 解绑、拖回贴靠带吸上（33ms 泵执行，见 pumps::sync_popout_snap）。
    pub pop_snap: Cell<bool>,
    pub pop_snap_off: Cell<(i32, i32)>,
    pub pop_snap_side: Cell<u8>,
    /// 当前正在播放的曲目路径（事件泵维护，波形上屏判断用）。
    pub current_path: RefCell<Option<PathBuf>>,
    /// 播放列表显示模型：主窗口抽屉与独立弹窗共享同一 ModelRc，
    /// 增删/过滤/排序只发生一次，两个窗口同步呈现。
    pub playlist_model: Rc<VecModel<PlaylistEntry>>,
    /// 播放列表独立窗口（阶段 C）：Some = 已弹出。关闭即销毁实例释放
    /// 全部 UI 资源（内存优先），下次弹出重建——列表内容在 Rust 侧，
    /// 重建零成本恢复。Slint 全局按组件实例隔离，跨窗口桥接见 pumps.rs。
    pub playlist_window: RefCell<Option<PlaylistWindow>>,
    /// 弹窗位置记忆（物理坐标）：会话内重开用，退出时随设置持久化。
    pub playlist_pop_pos: Cell<Option<(i32, i32)>>,
    /// 波形悬停提示的格式化缓存（上次 frac, duration）：输入不变就跳过
    /// format! 分配——泵内禁止每帧分配（性能预算），此提示每 33ms 触发。
    pub wave_tip_cache: Cell<(f32, f32)>,
}

/// 统一关闭流程：保存记忆设置、隐藏窗口并退出事件循环。
pub(crate) fn do_close(app: &App) {
    let transport = app.ui.global::<TransportState>();
    let playlist_state = app.ui.global::<PlaylistState>();
    let current = {
        let list = app.playlist.borrow();
        let idx = playlist_state.get_playlist_current();
        if idx >= 0 {
            list.get(idx as usize).cloned()
        } else {
            None
        }
    };
    // 独立播放列表窗口：先关闭（内部记录位置）再保存设置，位置随本次持久化。
    let pop_pos = close_playlist_window(app);
    save_settings(
        &app.playlist.borrow(),
        transport.get_position(),
        transport.get_volume(),
        app.mode_cell.get(),
        transport.get_always_on_top(),
        current.as_ref(),
        &app.eq.borrow(),
        pop_pos,
    );
    let _ = app.ui.window().hide();
    let _ = slint::quit_event_loop();
}

/// 打开播放列表独立窗口（幂等）：创建实例、共享列表模型、注册回调、
/// 恢复记忆位置、应用亚克力/圆角并子类化 WndProc（Alt+F4 收回弹窗）。
pub(crate) fn open_playlist_window(app: &Rc<App>) {
    if app.playlist_window.borrow().is_some() {
        return;
    }
    let Ok(pw) = PlaylistWindow::new() else {
        eprintln!("[sys] 创建播放列表窗口失败");
        return;
    };
    // 共享显示模型：弹窗的 PlaylistState.playlist 指向与主窗口相同的
    // ModelRc（全局按组件实例隔离，模型必须手动共享）。
    pw.global::<PlaylistState>()
        .set_playlist(ModelRc::from(Rc::clone(&app.playlist_model)));
    // 打开瞬间对齐当前曲目高亮；其余属性（主题色/播放状态/粒子时钟）
    // 由 33ms 泵逐帧同步，无需在此逐项搬运。
    let cur = app.ui.global::<PlaylistState>().get_playlist_current();
    pw.global::<PlaylistState>().set_playlist_current(cur);
    register_playlist_window_callbacks(app, &pw);
    pw.show().expect("显示播放列表窗口失败");
    // 亚克力/圆角与 WndProc 子类化。winit 窗口惰性创建：HWND 未就绪时
    // 短延时重试一次（与主窗口的 setup_timer 同一防御）。
    if hwnd_from_window(pw.window()).is_none() {
        let weak = pw.as_weak();
        let app_weak = Rc::downgrade(app);
        slint::Timer::single_shot(Duration::from_millis(50), move || {
            if let Some(pw) = weak.upgrade() {
                apply_system_effects(pw.window());
                setup_playlist_window(pw.window());
                // 定位也依赖窗口就绪（show 前/未就绪时 set_position 被静默
                // 丢弃，实测弹窗落在 winit 默认位置），故挂在同一重试点。
                if let Some(app) = app_weak.upgrade() {
                    place_popout_right_of_main(&app, &pw);
                }
            }
        });
    } else {
        apply_system_effects(pw.window());
        setup_playlist_window(pw.window());
        place_popout_right_of_main(app, &pw);
    }
    app.playlist_window.borrow_mut().replace(pw);
    app.ui.global::<PlaylistState>().set_popped(true);
    eprintln!("[sys] 播放列表已弹出为独立窗口 playlist-opened");
}

/// 将弹窗定位到主窗口右侧（用户要求：每次弹出都贴主窗右缘 +8px，不做绝对
/// 位置记忆）。主窗矩形用 Win32 GetWindowRect 取物理像素；屏幕右缘放不下
/// 则翻到主窗左侧，纵向贴主窗顶缘并夹回屏内。必须在 winit 窗口就绪后调用。
fn place_popout_right_of_main(app: &App, pw: &PlaylistWindow) {
    let Some((ml, mt, mr, _mb)) = window_rect_px(app.ui.window()) else {
        return;
    };
    // 面板尺寸 340×520 逻辑 px，按弹窗所在显示器的缩放换算物理尺寸。
    let scale = pw.window().scale_factor();
    let w = (340.0 * scale) as i32;
    let h = (520.0 * scale) as i32;
    let (cx, cy) = screen_size();
    let mut x = mr + 8;
    if x + w > cx {
        x = ml - w - 8;
    }
    let mut y = mt;
    if y + h > cy {
        y = cy - h;
    }
    if y < 0 {
        y = 0;
    }
    pw.window().set_position(slint::PhysicalPosition::new(x, y));
    // 磁贴：弹出即贴靠（右侧布局是贴靠位；屏幕右缘放不下翻到左侧则贴
    // 左侧），记录相对偏移与方位（光效用）供 33ms 泵联动。
    app.pop_snap.set(true);
    app.pop_snap_off.set((x - ml, y - mt));
    app.pop_snap_side.set(if x >= mr { 1 } else { 2 });
}

/// 关闭播放列表独立窗口（幂等）：记录位置（供会话内重开与退出持久化）、
/// 复位共享显示模型的搜索过滤并销毁实例，返回记录的位置。
pub(crate) fn close_playlist_window(app: &App) -> Option<(i32, i32)> {
    let pw = app.playlist_window.borrow_mut().take()?;
    let pos = pw.window().position();
    app.playlist_pop_pos.set(Some((pos.x, pos.y)));
    // 弹窗内的搜索框属于它自己的全局实例（随窗口销毁），但共享显示模型
    // 的过滤必须复位，否则抽屉重新打开仍是被过滤状态。
    app.playlist_view
        .borrow_mut()
        .set_filter("", &app.playlist.borrow());
    app.ui.global::<PlaylistState>().set_popped(false);
    eprintln!("[sys] 播放列表独立窗口已收回 playlist-closed");
    // Slint 事件循环对“显示中”的窗口持强引用：不先 hide() 直接 drop 会留下
    // 幽灵窗口——仍然可见但脱离管理，点 ✕ 只会去开抽屉、Alt+F4 落在空引用上
    // （“弹窗像死了一样、无法关闭”的根因之一）。hide 后事件循环解除持有，
    // 下一行的 drop 才会真正销毁窗口并同步释放全部 UI 资源。
    let _ = pw.hide();
    Some((pos.x, pos.y)) // pw 在此 drop：窗口与全部 UI 资源同步释放
}

/// 程序入口：装配一切并阻塞在 UI 事件循环上。
pub fn run() {
    // 单例模式：已有实例时转发文件并退出，不创建第二个窗口。
    enforce_single_instance();

    let ui = MainWindow::new().expect("创建窗口失败");
    let audio = Rc::new(AudioEngine::start());
    let (wave_tx, wave_rx) = spawn_waveform_worker();
    let (file_tx, file_rx) = mpsc::channel::<FileEvent>();
    set_file_events(file_tx.clone());
    // 更新流程事件通道：检查/下载线程产出，事件泵消费。
    let (update_tx, update_rx) = mpsc::channel::<UpdateEvent>();

    // —— 恢复记忆设置 ——
    let settings = load_settings();
    let transport = ui.global::<TransportState>();
    let playlist_state = ui.global::<PlaylistState>();
    let about_state = ui.global::<AboutState>();
    transport.set_volume(settings.volume);
    transport.set_volume_text(SharedString::from(format!(
        "{}%",
        (settings.volume * 100.0).round() as u32
    )));
    transport.set_mode_text(settings.mode.label().into());
    audio.send(Command::SetVolume(settings.volume));
    audio.send(Command::SetMode(settings.mode));
    // 均衡器参数下发（EqState 面板接线见 ui_callbacks，settings.txt eq= 持久化）。
    audio.set_eq(settings.eq.clone());
    // EQ 面板初始增益模型：来自 settings.txt 的 eq= 行（10 段 dB 值）。
    let eq_gains_model: Rc<VecModel<f32>> = Rc::new(VecModel::from(settings.eq.gains.to_vec()));
    let eq_state = ui.global::<EqState>();
    eq_state.set_gains(ModelRc::from(Rc::clone(&eq_gains_model)));
    eq_state.set_enabled(settings.eq.enabled);
    // 关于界面展示的版本号（单一来源：Cargo.toml）。
    about_state.set_version(app_version().into());

    // 波形条模型：整个运行期只建一次，换曲时逐行更新数据，
    // Slint 侧复用行元素并触发高度过渡动画，避免整排重建。
    let wave_bars_model: Rc<VecModel<f32>> = Rc::new(VecModel::from(Vec::new()));
    transport.set_wave_bars(ModelRc::from(Rc::clone(&wave_bars_model)));

    // 播放列表（仅保留仍存在的文件，避免启动后大量报错）。
    let playlist: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));
    let playlist_model: Rc<VecModel<PlaylistEntry>> = Rc::new(VecModel::default());
    let playlist_view = Rc::new(RefCell::new(PlaylistView::new(Rc::clone(&playlist_model))));
    for path in &settings.playlist {
        if path.is_file() {
            playlist.borrow_mut().push(path.clone());
        }
    }
    playlist_view.borrow_mut().rebuild(&playlist.borrow());
    playlist_state.set_playlist(ModelRc::from(playlist_model.clone()));
    audio.send(Command::SetPlaylist(playlist.borrow().clone()));

    let waveform_cache: Rc<RefCell<HashMap<PathBuf, WaveformResult>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let cache_order: Rc<RefCell<VecDeque<PathBuf>>> = Rc::new(RefCell::new(VecDeque::new()));
    let mode_cell: Rc<Cell<PlaybackMode>> = Rc::new(Cell::new(settings.mode));
    // 跳转等待：Some((目标秒, 发起时刻))。松手后 UI 已乐观更新到目标，
    // 期间忽略播放引擎尚未完成 seek 前残留的旧位置上报。
    let seek_wait: Rc<RefCell<Option<SeekState>>> = Rc::new(RefCell::new(None));
    // 背景交叉淡入状态：当前可见层是否为 front。
    let bg_front = Rc::new(Cell::new(true));
    // 主题色补间（换曲时约 400ms 颜色过渡，见 ThemeTween）。
    // 初始目标与 main.slint 的默认 theme-color 一致。
    let theme_tween = Rc::new(ThemeTween::new([0x5a, 0xc8, 0xfa]));
    // 模式提示 / 音量弹层的自动隐藏计时器。
    let mode_hide_timer = Rc::new(slint::Timer::default());
    let popup_hide_timer = Rc::new(slint::Timer::default());

    ui.show().expect("显示窗口失败");

    let app = Rc::new(App {
        ui,
        audio,
        playlist,
        playlist_view,
        wave_bars_model,
        waveform_cache,
        cache_order,
        mode_cell,
        seek_wait,
        bg_front,
        theme_tween,
        file_tx,
        file_rx,
        wave_tx,
        wave_rx,
        update_tx,
        update_rx,
        mode_hide_timer,
        popup_hide_timer,
        eq: RefCell::new(settings.eq.clone()),
        eq_gains_model: Rc::clone(&eq_gains_model),
        pop_snap: Cell::new(false),
        pop_snap_off: Cell::new((0, 0)),
        pop_snap_side: Cell::new(0),
        current_path: RefCell::new(None),
        playlist_model: Rc::clone(&playlist_model),
        playlist_window: RefCell::new(None),
        playlist_pop_pos: Cell::new(settings.pop_pos),
        wave_tip_cache: Cell::new((-1.0, 0.0)),
    });

    // winit 窗口是惰性创建的：事件循环启动（Resumed 阶段）后才真正存在，
    // 此前 window_handle() 返回 Unavailable。因此亚克力/圆角/拖拽注册等
    // 系统效果须等到窗口就绪后再应用（轮询检测，成功后停止）。
    let setup_timer = Rc::new(slint::Timer::default());
    {
        let app_weak = Rc::downgrade(&app);
        let stop_handle = Rc::clone(&setup_timer);
        setup_timer.start(
            slint::TimerMode::Repeated,
            Duration::from_millis(50),
            move || {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                if hwnd_from_window(app.ui.window()).is_none() {
                    return; // 窗口尚未创建，稍后重试。
                }
                stop_handle.stop();
                eprintln!("[sys] 窗口已创建，开始应用系统效果");
                apply_system_effects(app.ui.window());
                setup_drag_drop(app.ui.window());
            },
        );
    }

    // 启动 30 秒后静默检查一次更新：失败无感；发现新版本仅在关于按钮
    // 加小圆点、关于界面内呈现，不弹窗、不自动下载。
    let probe_timer = slint::Timer::default();
    {
        let app_weak = Rc::downgrade(&app);
        probe_timer.start(
            slint::TimerMode::SingleShot,
            Duration::from_secs(30),
            move || {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                let about_state = app.ui.global::<AboutState>();
                if about_state.get_update_state() == UpdateState::Idle {
                    events::spawn_update_check(app.update_tx.clone(), false);
                }
            },
        );
    }

    // 模式提示 / 音量弹层的自动隐藏计时器初次武装（此后由回调 restart）。
    {
        let app_weak = Rc::downgrade(&app);
        app.mode_hide_timer.start(
            slint::TimerMode::SingleShot,
            Duration::from_millis(1600),
            move || {
                if let Some(app) = app_weak.upgrade() {
                    app.ui.global::<TransportState>().set_mode_showing(false);
                }
            },
        );
    }
    {
        let app_weak = Rc::downgrade(&app);
        app.popup_hide_timer.start(
            slint::TimerMode::SingleShot,
            Duration::from_millis(3000),
            move || {
                if let Some(app) = app_weak.upgrade() {
                    app.ui
                        .global::<TransportState>()
                        .set_volume_popup_open(false);
                }
            },
        );
    }

    // 粒子系统：每 33ms 推进相位，驱动白色粒子与列表均衡器动画；
    // 同时推进主题色补间（换曲颜色过渡）。暂停时粒子相位冻结，
    // 工具栏悬停仅在状态变化时写属性，避免无谓的重绘。
    let particle_timer = slint::Timer::default();
    {
        let app_weak = Rc::downgrade(&app);
        particle_timer.start(
            slint::TimerMode::Repeated,
            Duration::from_millis(33),
            move || {
                if let Some(app) = app_weak.upgrade() {
                    particle_33ms(&app);
                }
            },
        );
    }
    // UI 线程看门狗：事件循环停滞时输出诊断（后台线程，不干扰 UI）。
    spawn_ui_watchdog();

    // —— 回调接线（按领域分组，见 ui_callbacks）——
    register_callbacks(&app);

    // 恢复置顶状态与上次播放进度；测试钩子（环境变量）直达对应界面。
    if settings.pin {
        let transport = app.ui.global::<TransportState>();
        transport.set_always_on_top(true);
        set_always_on_top(app.ui.window(), true);
    }
    // 测试辅助：ZZH_OPEN_PLAYLIST=1 启动时直接展开播放列表抽屉。
    if std::env::var("ZZH_OPEN_PLAYLIST").as_deref() == Ok("1") {
        app.ui.global::<PlaylistState>().set_playlist_open(true);
        set_playlist_open(true);
    }
    // 测试辅助：ZZH_OPEN_SEARCH=1 启动时直接展开播放列表搜索框。
    if std::env::var("ZZH_OPEN_SEARCH").as_deref() == Ok("1") {
        app.ui.global::<PlaylistState>().set_search_open(true);
    }
    // 测试辅助：ZZH_OPEN_ABOUT=1 启动时直接打开“关于”对话框。
    if std::env::var("ZZH_OPEN_ABOUT").as_deref() == Ok("1") {
        app.ui.global::<AboutState>().set_about_open(true);
        set_about_open(true);
    }
    // 测试辅助：ZZH_OPEN_POPOUT=1 启动时直接弹出播放列表独立窗口。
    if std::env::var("ZZH_OPEN_POPOUT").as_deref() == Ok("1") {
        open_playlist_window(&app);
    }
    if let Some(cur) = &settings.current
        && let Some(idx) = app.playlist.borrow().iter().position(|p| p == cur)
    {
        app.ui
            .global::<PlaylistState>()
            .set_playlist_current(idx as i32);
        app.audio.send(Command::PlayAt(idx));
        if settings.position > 1.0 {
            app.audio
                .send(Command::Seek(Duration::from_secs_f32(settings.position)));
        }
    }

    // 启动参数（如“打开方式”传入的音乐文件）加入播放列表；
    // 首个文件立即播放——双击文件打开时用户意图明确是听这首，而非接着上次继续。
    let mut args = std::env::args().skip(1).peekable();
    if args.peek().is_some() {
        let first = PathBuf::from(args.next().unwrap());
        if first.is_file() {
            play_file_now(
                &first,
                &app.playlist,
                &app.playlist_view,
                &app.ui.global::<PlaylistState>(),
                &app.audio,
            );
        }
    }
    for arg in args {
        let path = PathBuf::from(arg);
        if path.is_dir() {
            events::spawn_folder_scan(path, app.file_tx.clone());
        } else if path.is_file() {
            let _ = add_track(
                path,
                &app.playlist,
                &app.playlist_view,
                &app.ui.global::<PlaylistState>(),
                &app.audio,
            );
        }
    }

    // —— 16ms 周期泵：音频事件 / 文件事件 / 波形结果 / 更新事件 ——
    // （提速自 100ms：seek 回执与换曲状态的感知延迟降至一帧，见 pumps 模块说明。）
    let pump_timer = slint::Timer::default();
    {
        let app_weak = Rc::downgrade(&app);
        pump_timer.start(
            slint::TimerMode::Repeated,
            Duration::from_millis(16),
            move || {
                if let Some(app) = app_weak.upgrade() {
                    pump_events(&app);
                }
            },
        );
    }

    // 波形磁盘缓存维护（孤儿清理 + LRU 上限）放到后台线程，不阻塞启动。
    let _ = std::thread::Builder::new()
        .name("wavecache-trim".to_string())
        .spawn(trim_wave_cache);

    app.ui.run().expect("UI 事件循环失败");
}
