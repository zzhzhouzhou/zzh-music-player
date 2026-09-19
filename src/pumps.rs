//! UI 线程的两只周期泵。
//! 16ms 事件泵：音频事件 / 文件事件 / 波形结果 / 更新事件 四个 mpsc 的统一消费点。
//! （旧为 100ms：慢泵让 seek 回执、换曲状态、文件追加的感知延迟高达 100ms；
//! 固定 16ms 与显示刷新对齐，四个 try_recv 的空转成本可忽略。不用动态调间隔——
//! Slint Timer 每次 restart 需重新装箱闭包，违反“泵内零分配”约束。）
//! 33ms 泵：粒子时钟、主题色补间、工具栏悬停、拖拽排序自动滚动、
//! 弹窗磁贴联动。
//! 约束：泵内不做每帧分配的重活；属性只在值变化时写入（先读后写比对）。

use std::time::Instant;

use slint::{ComponentHandle, Image, Model, SharedString};

use crate::app::App;
use crate::audio_engine::{Command, Event};
use crate::events::{FileEvent, UpdateEvent, spawn_update_download};
use crate::playlist::{add_track, add_tracks_batch, open_file_dialog, play_file_now};
use crate::render_utils::{format_time, placeholder_bars, push_background};
use crate::transport::{SEEK_CONFIRM_TIMEOUT, SEEK_SETTLE_WINDOW, SeekState};
use crate::waveform::{apply_waveform, cache_insert, prefetch_next_track};
use crate::waveform_cache::read_wave_cache;
use crate::windows_platform::{
    cursor_position, is_about_open, left_button_down, popout_dragging, set_about_open,
    window_rect_px,
};
use crate::{AboutState, PlaylistState, TransportState, UpdateState};

/// 16ms 事件泵：四个通道的统一消费点。轮询顺序（音频→文件→波形→更新）
/// 与拆分前保持一致：音频状态优先上屏，文件操作其次，波形与更新最后。
pub fn pump_events(app: &App) {
    let transport = app.ui.global::<TransportState>();
    let playlist_state = app.ui.global::<PlaylistState>();
    let about_state = app.ui.global::<AboutState>();
    sync_popout_current(app, &playlist_state);
    drain_audio(app, &transport, &playlist_state);
    drain_files(app, &transport, &playlist_state);
    drain_waves(app, &transport, &playlist_state);
    drain_updates(app, &about_state);
}

/// 独立弹窗桥接：当前曲目高亮。Slint 全局按组件实例隔离（见
/// playlist_window.slint），弹窗的 PlaylistState 是另一份实例，播放/增删/
/// 排序引起的高亮变化由这里从主窗口实例单向同步（仅变化时写入）。
fn sync_popout_current(app: &App, playlist_state: &PlaylistState) {
    let guard = app.playlist_window.borrow();
    let Some(pw) = guard.as_ref() else {
        return;
    };
    let pw_state = pw.global::<PlaylistState>();
    let cur = playlist_state.get_playlist_current();
    if pw_state.get_playlist_current() != cur {
        pw_state.set_playlist_current(cur);
    }
}

/// 消费音频引擎事件：播放状态、位置、跳转回执与错误。
fn drain_audio(app: &App, transport: &TransportState, playlist_state: &PlaylistState) {
    while let Some(event) = app.audio.try_recv_event() {
        match event {
            Event::TrackStarted { path } => {
                *app.current_path.borrow_mut() = Some(path.clone());
                transport.set_playing(true);
                transport.set_position(0.0);
                transport.set_position_text(format_time(0.0));
                // 新曲目开始：上一首的跳转等待与锁定作废。
                *app.seek_wait.borrow_mut() = None;
                transport.set_seek_lock(false);
                transport.set_dragging(false);
                let idx = app.playlist.borrow().iter().position(|p| *p == path);
                playlist_state.set_playlist_current(idx.map(|i| i as i32).unwrap_or(-1));
                if let Some(res) = app.waveform_cache.borrow().get(&path) {
                    // 内存缓存命中时直接复用，切歌几乎无感。
                    apply_waveform(
                        transport,
                        res,
                        &app.wave_bars_model,
                        &app.bg_front,
                        &app.theme_tween,
                    );
                    prefetch_next_track(
                        playlist_state,
                        &app.playlist,
                        &app.waveform_cache,
                        &app.wave_tx,
                    );
                } else if let Some(res) = read_wave_cache(&path) {
                    // 磁盘缓存命中：免整曲解码，元数据/封面/波形一步到位。
                    {
                        let mut cache = app.waveform_cache.borrow_mut();
                        let mut order = app.cache_order.borrow_mut();
                        cache_insert(&mut cache, &mut order, path.clone(), res);
                    }
                    let res = app.waveform_cache.borrow().get(&path).cloned();
                    if let Some(res) = res.as_ref() {
                        apply_waveform(
                            transport,
                            res,
                            &app.wave_bars_model,
                            &app.bg_front,
                            &app.theme_tween,
                        );
                        prefetch_next_track(
                            playlist_state,
                            &app.playlist,
                            &app.waveform_cache,
                            &app.wave_tx,
                        );
                    }
                } else {
                    // 音频已开始播放，波形分析在后台进行。先显示轻量占位波形，
                    // 不让用户等分析完成才看到可操作的进度区；结果回来后再平滑替换。
                    // 背景交叉淡出到兜底深色，避免残留上一首的色调。
                    let _ = app.wave_tx.send(path.clone());
                    let title = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    transport.set_track_title(title.into());
                    transport.set_track_artist(SharedString::default());
                    app.wave_bars_model.set_vec(placeholder_bars());
                    transport.set_cover_image(Image::default());
                    transport.set_has_cover(false);
                    push_background(transport, Image::default(), &app.bg_front);
                }
                eprintln!("开始播放: {:?}", path);
            }
            Event::Duration { duration } => {
                // 解码器可立即提供时长；无需等待完整波形分析。
                let seconds = duration.as_secs_f32();
                transport.set_duration(seconds);
                transport.set_duration_text(format_time(seconds));
            }
            Event::SeekApplied { position } => {
                // rodio 已完成 seek；保持目标一个短窗口，吸收已经排队的旧
                // Position。锁定型跳转（点击/拖拽）显示继续钉在目标上；
                // 非锁定型（方向键）不钉显示，由插值动画平滑到位。
                let seconds = position.as_secs_f32();
                let (target, lock) = match *app.seek_wait.borrow() {
                    Some(SeekState::Pending { target, lock, .. })
                    | Some(SeekState::Settling { target, lock, .. }) => (target, lock),
                    None => {
                        // 无在途跳转（如启动恢复进度）：按实际落点钉住，
                        // 避免锁定期间显示回落到默认的 0。
                        let frac = if transport.get_duration() > 0.0 {
                            (seconds / transport.get_duration()).min(1.0)
                        } else {
                            0.0
                        };
                        transport.set_seek_lock_frac(frac);
                        (seconds, true)
                    }
                };
                *app.seek_wait.borrow_mut() = Some(SeekState::Settling {
                    target,
                    until: Instant::now() + SEEK_SETTLE_WINDOW,
                    lock,
                });
                transport.set_seek_lock(lock);
                transport.set_position(target);
                transport.set_position_text(format_time(target));
            }
            Event::Position(pos) => {
                let pos = pos.as_secs_f32();
                // seek 生效前与刚生效后的陈旧上报一律顶替为目标值：
                // 锁定型显示钉在目标；非锁定型 position=target 让
                // 插值动画从当前位置平滑滑向目标。等待超时则放行真实位置。
                let applied = {
                    let mut wait = app.seek_wait.borrow_mut();
                    match *wait {
                        Some(SeekState::Pending { target, since, .. })
                            if since.elapsed() <= SEEK_CONFIRM_TIMEOUT =>
                        {
                            target
                        }
                        Some(SeekState::Settling { target, until, .. })
                            if Instant::now() < until =>
                        {
                            target
                        }
                        _ => {
                            *wait = None;
                            transport.set_seek_lock(false);
                            pos
                        }
                    }
                };
                transport.set_position(applied);
                transport.set_position_text(format_time(applied));
            }
            Event::Finished => {
                transport.set_playing(false);
                transport.set_position(transport.get_duration());
                transport.set_position_text(format_time(transport.get_duration()));
                *app.seek_wait.borrow_mut() = None;
                transport.set_seek_lock(false);
                transport.set_dragging(false);
                *app.current_path.borrow_mut() = None;
                playlist_state.set_playlist_current(-1);
            }
            Event::Error(e) => {
                // 跳转失败：解除预览锁定，进度条回到真实位置。
                *app.seek_wait.borrow_mut() = None;
                transport.set_seek_lock(false);
                transport.set_dragging(false);
                eprintln!("音频错误: {e}");
            }
        }
    }
}

/// 消费文件事件：拖入/转发/双击/滚轮/关闭。
fn drain_files(app: &App, transport: &TransportState, playlist_state: &PlaylistState) {
    while let Ok(evt) = app.file_rx.try_recv() {
        match evt {
            FileEvent::Dropped(paths) => {
                for path in paths {
                    if path.is_dir() {
                        // 文件夹：后台线程递归扫描，每 50 个一批渐进式追加，
                        // 大文件夹也能立刻看到列表在增长。
                        crate::events::spawn_folder_scan(path, app.file_tx.clone());
                    } else {
                        let _ = add_track(
                            path,
                            &app.playlist,
                            &app.playlist_view,
                            playlist_state,
                            &app.audio,
                        );
                    }
                }
            }
            FileEvent::DroppedBatch(paths) => {
                add_tracks_batch(
                    &paths,
                    &app.playlist,
                    &app.playlist_view,
                    playlist_state,
                    &app.audio,
                );
            }
            FileEvent::OpenFiles(paths) => {
                // 第二个实例转发的“打开方式”文件：首个立即播放（即使已在列表中），其余仅加入列表。
                let mut files = paths.iter();
                if let Some(first) = files.next() {
                    play_file_now(
                        first,
                        &app.playlist,
                        &app.playlist_view,
                        playlist_state,
                        &app.audio,
                    );
                }
                for path in files {
                    let _ = add_track(
                        path.clone(),
                        &app.playlist,
                        &app.playlist_view,
                        playlist_state,
                        &app.audio,
                    );
                }
            }
            FileEvent::DoubleClick => open_file_dialog(
                &app.playlist,
                &app.playlist_view,
                playlist_state,
                &app.audio,
            ),
            FileEvent::Wheel(delta) => {
                let step = (delta as f32 / 120.0) * 0.05;
                let volume = (transport.get_volume() + step).clamp(0.0, 1.0);
                transport.set_volume(volume);
                transport.set_volume_text(slint::SharedString::from(format!(
                    "{}%",
                    (volume * 100.0).round() as u32
                )));
                transport.set_volume_popup_open(true);
                app.audio.send(Command::SetVolume(volume));
                app.popup_hide_timer.restart();
            }
            FileEvent::CloseRequest => {
                // 保存设置并退出（拦截了系统 WM_CLOSE）。
                crate::app::do_close(app);
            }
            FileEvent::PlaylistWindowClose => {
                // 弹窗的关闭按钮 / Alt+F4：只收回弹窗，绝不退出程序。
                crate::app::close_playlist_window(app);
            }
        }
    }
}

/// 消费波形结果：全部入 RAM 缓存，属于当前曲目的立即上屏并预取下一首。
fn drain_waves(app: &App, transport: &TransportState, playlist_state: &PlaylistState) {
    while let Ok(res) = app.wave_rx.try_recv() {
        // 只把属于当前曲目的波形立即上屏；其余缓存，等切到该曲再显示。
        let is_current = app
            .current_path
            .borrow()
            .as_ref()
            .is_some_and(|p| *p == res.path);
        {
            let mut cache = app.waveform_cache.borrow_mut();
            let mut order = app.cache_order.borrow_mut();
            cache_insert(&mut cache, &mut order, res.path.clone(), res);
        }
        if is_current {
            let cache = app.waveform_cache.borrow();
            let cur = app.current_path.borrow();
            if let Some(cached) = cache.get(cur.as_ref().unwrap().as_path()) {
                apply_waveform(
                    transport,
                    cached,
                    &app.wave_bars_model,
                    &app.bg_front,
                    &app.theme_tween,
                );
                drop(cache);
                prefetch_next_track(
                    playlist_state,
                    &app.playlist,
                    &app.waveform_cache,
                    &app.wave_tx,
                );
            }
        }
    }
}

/// 消费更新流程事件：驱动关于界面的状态机。
fn drain_updates(app: &App, about_state: &AboutState) {
    while let Ok(ev) = app.update_rx.try_recv() {
        match ev {
            UpdateEvent::Checking => {
                about_state.set_update_state(UpdateState::Checking);
                about_state.set_update_note(SharedString::default());
            }
            UpdateEvent::UpToDate => {
                about_state.set_update_state(UpdateState::Latest);
                about_state.set_update_note("已是最新版本".into());
            }
            UpdateEvent::Available {
                version,
                auto_download,
            } => {
                about_state.set_update_latest_version(SharedString::from(version.clone()));
                if auto_download {
                    about_state.set_update_state(UpdateState::Downloading);
                    about_state.set_update_progress(0.0);
                    about_state.set_update_note(format!("正在下载 v{version}…").into());
                    spawn_update_download(app.update_tx.clone());
                } else {
                    about_state.set_update_state(UpdateState::Available);
                    about_state.set_update_note(format!("发现新版本 v{version}").into());
                }
            }
            UpdateEvent::Progress(frac) => {
                about_state.set_update_progress(frac);
                about_state.set_update_note(
                    format!(
                        "正在下载 v{}… {:.0}%",
                        about_state.get_update_latest_version(),
                        frac * 100.0
                    )
                    .into(),
                );
            }
            UpdateEvent::Ready => {
                about_state.set_update_state(UpdateState::Ready);
                about_state.set_update_progress(1.0);
                about_state.set_update_note("新版本已就绪".into());
            }
            UpdateEvent::Failed(e) => {
                about_state.set_update_state(UpdateState::Failed);
                about_state.set_update_note(e.into());
            }
        }
    }
}

/// 33ms 泵：粒子时钟、关于开关同步、波形悬停提示、工具栏悬停与
/// 拖拽排序的浮块跟随 / 边缘自动滚动 / 悬停清理。
/// UI 线程心跳计数：33ms 泵每次推进 +1。看门狗后台线程据此判断
/// 事件循环是否停滞（弹窗"假死"类问题的现场证据采集点）。
static PUMP_TICK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// UI 线程看门狗：心跳停止推进超过 1 秒时输出诊断日志并周期性提醒。
/// 后台线程零锁零分配，不干扰事件循环；正常时完全静默。
pub fn spawn_ui_watchdog() {
    let _ = std::thread::Builder::new()
        .name("ui-watchdog".to_string())
        .spawn(|| {
            let mut last = PUMP_TICK.load(std::sync::atomic::Ordering::Relaxed);
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let now = PUMP_TICK.load(std::sync::atomic::Ordering::Relaxed);
                if now == last {
                    eprintln!("[watchdog] UI 线程心跳停止（>1s），事件循环疑似阻塞");
                }
                last = now;
            }
        });
}

pub fn particle_33ms(app: &App) {
    PUMP_TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let ui = &app.ui;
    let transport = ui.global::<TransportState>();
    let playlist_state = ui.global::<PlaylistState>();
    let about_state = ui.global::<AboutState>();
    app.theme_tween.tick(&transport, 0.033);
    // “关于”打开状态同步给 WndProc（滚轮隔离判断用）。
    let about = about_state.get_about_open();
    if is_about_open() != about {
        set_about_open(about);
    }
    // 波形悬停时间提示：输入（frac, duration）都没变就整段跳过——format!
    // 是逐帧分配，性能预算禁止空转；仅输入变化时才格式化并写属性。
    let frac = transport.get_wave_hover_frac();
    let duration = transport.get_duration();
    let last = app.wave_tip_cache.get();
    if frac != last.0 || duration != last.1 {
        app.wave_tip_cache.set((frac, duration));
        let tip = if frac >= 0.0 && duration > 0.0 {
            slint::SharedString::from(format!(
                "{} / {}",
                format_time(frac * duration),
                transport.get_duration_text()
            ))
        } else {
            slint::SharedString::from("")
        };
        if tip != transport.get_tooltip_text() {
            transport.set_tooltip_text(tip);
        }
    }
    if transport.get_playing() {
        let t = transport.get_particle_time() + 0.033;
        transport.set_particle_time(if t >= 1.0 { t - 1.0 } else { t });
    }
    // 工具栏悬停检测（仅主窗口）：光标进入工具栏矩形范围时让背景变实。
    if let Some((cx, cy)) = cursor_position() {
        {
            let scale = ui.window().scale_factor();
            let origin = ui.window().position();
            let logical_w = ui.window().size().width as f32 / scale;
            let logical_h = ui.window().size().height as f32 / scale;
            // 与 main.slint 的 control_bar（300×38、水平居中、距底 8px）保持一致。
            let bar_x = (logical_w - 330.0) / 2.0;
            let bar_y = logical_h - 46.0;
            let x0 = origin.x + (bar_x * scale) as i32;
            let x1 = origin.x + ((bar_x + 330.0) * scale) as i32;
            let y0 = origin.y + (bar_y * scale) as i32;
            let y1 = origin.y + ((bar_y + 38.0) * scale) as i32;
            let hovered = cx >= x0 && cx <= x1 && cy >= y0 && cy <= y1;
            if hovered != transport.get_toolbar_hovered() {
                transport.set_toolbar_hovered(hovered);
            }
        }
        // —— 独立播放列表窗口桥接（Slint 全局按组件实例隔离）——
        // 主题色 / 播放状态 / 粒子时钟逐帧从主窗口实例同步，仅变化时写入；
        // 列表内容靠共享模型，当前曲目高亮由事件泵同步。
        let pop_guard = app.playlist_window.borrow();
        if let Some(pw) = pop_guard.as_ref() {
            let pw_transport = pw.global::<TransportState>();
            let theme = transport.get_theme_color();
            if pw_transport.get_theme_color() != theme {
                pw_transport.set_theme_color(theme);
            }
            let playing = transport.get_playing();
            if pw_transport.get_playing() != playing {
                pw_transport.set_playing(playing);
            }
            if playing {
                let t = transport.get_particle_time();
                if pw_transport.get_particle_time() != t {
                    pw_transport.set_particle_time(t);
                }
            }
            // 弹窗承载列表：拖拽/悬停按弹窗几何换算。
            sync_playlist_pointer(&pw.global::<PlaylistState>(), pw.window(), cx, cy, true);
        } else {
            // 抽屉承载列表（弹窗未打开时二者互斥）。
            sync_playlist_pointer(
                &playlist_state,
                ui.window(),
                cx,
                cy,
                playlist_state.get_playlist_open(),
            );
        }
    }
    // 磁贴联动：贴靠中的弹窗跟随主窗移动（弹窗未打开时是纯读跳过，零成本）。
    sync_popout_snap(app);
}

/// 磁贴（snap）联动：贴靠中的弹窗每拍对齐主窗——主窗是自算拖动（无
/// 模态循环），泵在主窗拖动中照常 tick，因此弹窗**实时**跟随主窗移动，
/// 而不是等主窗停下再归位。拖离贴靠位超阈值解绑；未贴靠时拖回主窗
/// 四边贴靠带（右/左/下/上）自动吸上。弹窗自己的原生拖动是模态循环
/// （popout_dragging），泵不能抢窗口位置，整段让路。每拍成本：两次
/// GetWindowRect + 一次比较，可忽略。
fn sync_popout_snap(app: &App) {
    if left_button_down() && popout_dragging() {
        return; // 弹窗正在被原生拖动（模态循环），松手后的下一拍再判定
    }
    let guard = app.playlist_window.borrow();
    let Some(pw) = guard.as_ref() else {
        return;
    };
    let Some((ml, mt, mr, mb)) = window_rect_px(app.ui.window()) else {
        return;
    };
    let Some((pl, pt, pr, pb)) = window_rect_px(pw.window()) else {
        return;
    };
    const RELEASE: i32 = 48; // 拖弹窗离贴靠位超过此距离解绑
    const MAGNET: i32 = 28; // 贴靠带宽度（松手时吸上）
    let mut side = app.pop_snap_side.get();
    if app.pop_snap.get() {
        let (ox, oy) = app.pop_snap_off.get();
        let (dx, dy) = (pl - (ml + ox), pt - (mt + oy));
        if dx != 0 || dy != 0 {
            if dx.abs() > RELEASE || dy.abs() > RELEASE {
                // 弹窗被拖离贴靠位：解绑（光效随之熄灭）。
                app.pop_snap.set(false);
                side = 0;
            } else {
                // 主窗动了：贴着跟过去（拖动中每拍对齐 = 实时联动）。
                pw.window()
                    .set_position(slint::PhysicalPosition::new(ml + ox, mt + oy));
            }
        }
    } else {
        // 未贴靠：弹窗落在主窗任一边的贴靠带内（且与该边有重叠投影）则吸上。
        let v_overlap = pt < mb && pb > mt;
        let h_overlap = pl < mr && pr > ml;
        let near_right = (pl - (mr + 8)).abs() <= MAGNET && v_overlap;
        let near_left = ((pr + 8) - ml).abs() <= MAGNET && v_overlap;
        let near_bottom = (pt - (mb + 8)).abs() <= MAGNET && h_overlap;
        let near_top = ((pb + 8) - mt).abs() <= MAGNET && h_overlap;
        if near_right || near_left || near_bottom || near_top {
            app.pop_snap.set(true);
            app.pop_snap_off.set((pl - ml, pt - mt));
            side = if near_right {
                1
            } else if near_left {
                2
            } else if near_bottom {
                3
            } else {
                4
            };
        } else {
            side = 0;
        }
    }
    if side != app.pop_snap_side.get() {
        app.pop_snap_side.set(side);
        // 光效提示：贴靠方位同步给弹窗自己的全局实例（边缘亮起主题色辉光）。
        pw.global::<PlaylistState>().set_snap_side(side as i32);
    }
}

/// 拖拽排序浮块跟随 / 边缘自动滚动 / 悬停清理：把系统光标换算到指定窗口
/// 的局部坐标后驱动。抽屉与弹窗互斥打开，承载窗口由调用方决定；
/// list_visible = 该窗口的列表是否可交互（抽屉=playlist-open，弹窗恒真）。
fn sync_playlist_pointer(
    playlist_state: &PlaylistState,
    window: &slint::Window,
    cx: i32,
    cy: i32,
    list_visible: bool,
) {
    let scale = window.scale_factor();
    let origin = window.position();
    let local_x = (cx - origin.x) as f32 / scale;
    let local_y = (cy - origin.y) as f32 / scale;
    let logical_w = window.size().width as f32 / scale;
    let logical_h = window.size().height as f32 / scale;
    // 拖动排序浮块跟随：光标贴近列表上下缘时直接滚动视口（33ms 一拍）。
    if playlist_state.get_reorder_from() >= 0.0 {
        playlist_state.set_reorder_y(local_y);
        // 列表区：y 42..(logical_h - 6)；上/下缘 22px 内开始滚动，
        // 速度按深入边缘的程度最高 6px/拍（约 180px/s）。
        // 视口范围与面板一致：[-(vh-rows*32-2), 0]。
        const EDGE: f32 = 22.0;
        const MAX_SPEED: f32 = 6.0;
        const LIST_TOP: f32 = 42.0;
        const LIST_BOTTOM_GAP: f32 = 6.0;
        if playlist_state.get_reorder_to() >= 0.0 {
            let rows = playlist_state.get_playlist().row_count() as f32;
            let list_h = logical_h - 48.0;
            let vp_min = 0.0f32.min(list_h - (rows * 32.0 + 2.0));
            let vp = playlist_state.get_list_vp_y();
            let bottom = logical_h - LIST_BOTTOM_GAP;
            // 上缘向上滚（viewport-y 增大趋近 0），下缘向下滚（减小）。
            let delta = if local_y < LIST_TOP + EDGE {
                MAX_SPEED * (1.0 - (local_y - LIST_TOP) / EDGE).max(0.15)
            } else if local_y > bottom - EDGE {
                -MAX_SPEED * (1.0 - (bottom - local_y) / EDGE).max(0.15)
            } else {
                0.0
            };
            if delta != 0.0 {
                playlist_state.set_list_vp_y((vp + delta).max(vp_min).min(0.0));
            }
        }
    }
    // 光标离开列表区 / 列表关闭 / 正在拖动时清除行悬停高亮，
    // 避免覆盖层收不到“离开”事件导致的高亮滞留。
    // 列表区几何与 PlaylistPanel 的覆盖层保持一致。
    let in_list = list_visible
        && playlist_state.get_reorder_from() < 0.0
        && local_x >= 8.0
        && local_x <= logical_w - 8.0
        && local_y >= 42.0
        && local_y <= logical_h - 6.0;
    if !in_list {
        if playlist_state.get_hover_row() >= 0.0 {
            playlist_state.set_hover_row(-1.0);
        }
        if playlist_state.get_hover_button() != 0.0 {
            playlist_state.set_hover_button(0.0);
        }
    }
}
