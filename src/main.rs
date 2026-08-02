//! zzhMusicPlayer —— 极简原生 Rust + Slint 桌面音乐播放器。
//! 长条形窗口 + Windows 11 亚克力毛玻璃，仅含播放控制与波形进度条。

// Release 构建下隐藏控制台窗口（纯 GUI 应用）。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_engine;
mod waveform_generator;

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use audio_engine::{AudioEngine, Command, Event};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use slint::ComponentHandle;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{
    DwmSetWindowAttribute, DWMSBT_TRANSIENTWINDOW, DWMWA_SYSTEMBACKDROP_TYPE,
    DWMWA_USE_IMMERSIVE_DARK_MODE, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
};
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, GetCursorPos, GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWLP_WNDPROC,
    HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WM_DROPFILES,
};

slint::include_modules!();

/// 双击判定的最大时间间隔（毫秒）。
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);
/// 双击判定的位置容差（逻辑像素）。
const DOUBLE_CLICK_TOLERANCE: f32 = 8.0;
/// 波形结果缓存上限：拖入大量文件时只保留最近若干份，避免内存无限增长。
const WAVE_CACHE_LIMIT: usize = 8;

/// 波形生成结果（后台线程产出，UI 线程消费；SharedPixelBuffer 为 Send）。
struct WaveformResult {
    path: PathBuf,
    bg: SharedPixelBuffer<Rgba8Pixel>,
    fg: SharedPixelBuffer<Rgba8Pixel>,
    duration: Duration,
    title: Option<String>,
    artist: Option<String>,
}

/// 文件相关外部事件（OS 拖拽 / 双击），经通道由 UI 线程统一处理。
enum FileEvent {
    Dropped(Vec<PathBuf>),
    DoubleClick,
}

/// WndProc 与 UI 线程之间的文件事件通道。
static FILE_EVENTS: OnceLock<Sender<FileEvent>> = OnceLock::new();
/// 被替换的原窗口过程（winit 的 WndProc）。
static ORIGINAL_WNDPROC: OnceLock<isize> = OnceLock::new();

/// `SetWindowCompositionAttribute`（未文档化 API）的亚克力策略。
/// 结构布局参考 winapi 的 `ACCENT_POLICY`。
#[repr(C)]
struct AccentPolicy {
    accent_state: i32,
    accent_flags: u32,
    gradient_color: u32,
    animation_id: u32,
}

/// `SetWindowCompositionAttribute` 的属性数据。
#[repr(C)]
struct WindowCompositionAttribData {
    attribute: i32,
    data: *mut core::ffi::c_void,
    size_of_data: usize,
}

const WCA_ACCENT_POLICY: i32 = 19;
const ACCENT_ENABLE_ACRYLICBLURBEHIND: i32 = 4;

/// 启动波形后台线程：接收文件路径，解码生成波形位图与时长。
fn spawn_waveform_worker() -> (Sender<PathBuf>, Receiver<WaveformResult>) {
    let (job_tx, job_rx) = mpsc::channel::<PathBuf>();
    let (res_tx, res_rx) = mpsc::channel::<WaveformResult>();
    std::thread::Builder::new()
        .name("waveform".to_string())
        .spawn(move || {
            while let Ok(path) = job_rx.recv() {
                let result = waveform_generator::analyze(&path).map(|wf| {
                    let (bg, fg) = waveform_generator::render_wave_buffers(&wf.columns);
                    WaveformResult {
                        path,
                        bg,
                        fg,
                        duration: wf.duration,
                        title: wf.title,
                        artist: wf.artist,
                    }
                });
                match result {
                    Ok(res) => {
                        if res_tx.send(res).is_err() {
                            break;
                        }
                    }
                    Err(e) => eprintln!("波形生成失败: {e}"),
                }
            }
        })
        .expect("无法创建波形线程");
    (job_tx, res_rx)
}

/// 新文件即刻入队播放，同时交给后台线程生成波形（播放不等待波形）。
fn enqueue_file(path: PathBuf, audio: &AudioEngine, wave_tx: &Sender<PathBuf>) {
    audio.send(Command::Load(path.clone()));
    let _ = wave_tx.send(path);
}

/// 把波形结果应用到 UI（波形图、时长与歌曲元数据）。
fn apply_waveform(state: &UIState, res: &WaveformResult) {
    state.set_wave_bg_image(Image::from_rgba8(res.bg.clone()));
    state.set_wave_fg_image(Image::from_rgba8(res.fg.clone()));
    state.set_duration(res.duration.as_secs_f32());
    // 元数据缺失时退回文件名作为标题。
    let title = res
        .title
        .clone()
        .or_else(|| {
            res.path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .unwrap_or_default();
    state.set_track_title(title.into());
    state.set_track_artist(res.artist.clone().unwrap_or_default().into());
}

/// 从 `slint::Window` 获取原生 HWND。
fn hwnd_from_window(window: &slint::Window) -> Option<HWND> {
    let handle = window.window_handle();
    let rwh = handle.window_handle().ok()?;
    match rwh.as_raw() {
        RawWindowHandle::Win32(win32) => Some(win32.hwnd.get() as *mut _),
        _ => None,
    }
}

/// 应用 Windows 11 亚克力毛玻璃、深色着色与圆角。
///
/// 优先使用 DWM 系统背景（Win11 22H2+，`DWMSBT_TRANSIENTWINDOW` 即 Acrylic），
/// 失败则降级 `SetWindowCompositionAttribute`（Win10 20H1+ / Win11 全版本）。
fn apply_system_effects(window: &slint::Window) {
    let Some(hwnd) = hwnd_from_window(window) else { return };
    unsafe {
        // 深色亚克力着色（与深色 UI 一致）。
        let dark: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE as u32,
            &dark as *const i32 as *const _,
            std::mem::size_of::<i32>() as u32,
        );
        // 圆角（DWMWCP_ROUND）。
        let corner: i32 = DWMWCP_ROUND;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE as u32,
            &corner as *const i32 as *const _,
            std::mem::size_of::<i32>() as u32,
        );
        // 亚克力：DWM 系统背景。
        let backdrop: i32 = DWMSBT_TRANSIENTWINDOW;
        let hr = DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE as u32,
            &backdrop as *const i32 as *const _,
            std::mem::size_of::<i32>() as u32,
        );
        if hr != 0 {
            if apply_acrylic_fallback(hwnd) {
                eprintln!("[sys] 亚克力：SetWindowCompositionAttribute 降级成功");
            } else {
                eprintln!("[sys] 毛玻璃设置失败，已回退半透明背景");
            }
        } else {
            eprintln!("[sys] 亚克力：DWM system backdrop 已应用");
        }
    }
}

/// 降级亚克力：`SetWindowCompositionAttribute(ACCENT_ENABLE_ACRYLICBLURBEHIND)`。
/// 该 API 未在 SDK 中公开，故经 `GetProcAddress` 动态加载（Win10 20H1+ / Win11）。
fn apply_acrylic_fallback(hwnd: HWND) -> bool {
    use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
    type SetAccent = unsafe extern "system" fn(HWND, *mut WindowCompositionAttribData) -> i32;
    unsafe {
        let module = GetModuleHandleW(windows_sys::core::w!("user32.dll"));
        if module.is_null() {
            return false;
        }
        // GetProcAddress 接受 ANSI 名称或序号。
        let Some(proc) = GetProcAddress(module, c"SetWindowCompositionAttribute".as_ptr() as *const u8) else {
            return false;
        };
        let set_accent: SetAccent = std::mem::transmute(proc);
        let mut accent = AccentPolicy {
            accent_state: ACCENT_ENABLE_ACRYLICBLURBEHIND,
            accent_flags: 0,
            gradient_color: 0,
            animation_id: 0,
        };
        let mut data = WindowCompositionAttribData {
            attribute: WCA_ACCENT_POLICY,
            data: &mut accent as *mut AccentPolicy as *mut _,
            size_of_data: std::mem::size_of::<AccentPolicy>(),
        };
        // 返回 BOOL：非 0 表示成功。
        set_accent(hwnd, &mut data) != 0
    }
}

/// 置顶 / 取消置顶。
fn set_always_on_top(window: &slint::Window, on: bool) {
    let Some(hwnd) = hwnd_from_window(window) else { return };
    unsafe {
        let insert_after = if on { HWND_TOPMOST } else { HWND_NOTOPMOST };
        SetWindowPos(
            hwnd,
            insert_after,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );
    }
}

/// 读取鼠标在屏幕上的物理坐标（用于平滑拖动窗口）。
fn cursor_position() -> Option<(i32, i32)> {
    let mut pt = POINT { x: 0, y: 0 };
    unsafe { (GetCursorPos(&mut pt) != 0).then_some((pt.x, pt.y)) }
}

/// 收集 `WM_DROPFILES` 中的全部文件路径。
unsafe fn collect_dropped_files(drop: HDROP) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    unsafe {
        let count = DragQueryFileW(drop, u32::MAX, std::ptr::null_mut(), 0);
        for i in 0..count {
            let len = DragQueryFileW(drop, i, std::ptr::null_mut(), 0);
            let mut buf = vec![0u16; (len + 1) as usize];
            DragQueryFileW(drop, i, buf.as_mut_ptr(), len + 1);
            paths.push(PathBuf::from(String::from_utf16_lossy(&buf[..len as usize])));
        }
        DragFinish(drop);
    }
    paths
}

/// 子类化窗口过程：拦截文件拖拽，其余消息转发原过程。
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DROPFILES => {
            let paths = unsafe { collect_dropped_files(wparam as *mut _) };
            if let Some(tx) = FILE_EVENTS.get() {
                let _ = tx.send(FileEvent::Dropped(paths));
            }
            0
        }
        _ => {
            let original = ORIGINAL_WNDPROC.get().copied().unwrap_or(0);
            if original != 0 {
                let proc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT =
                    unsafe { std::mem::transmute(original) };
                unsafe { proc(hwnd, msg, wparam, lparam) }
            } else {
                unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
            }
        }
    }
}

/// 注册 OS 文件拖拽：`DragAcceptFiles` + WndProc 子类化。
fn setup_drag_drop(window: &slint::Window) {
    let Some(hwnd) = hwnd_from_window(window) else {
        eprintln!("[sys] 拖拽注册失败：获取 HWND 失败");
        return;
    };
    unsafe {
        let original = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
        if original == 0 {
            eprintln!("[sys] 拖拽注册失败：获取原 WndProc 失败");
            return;
        }
        let _ = ORIGINAL_WNDPROC.set(original);
        let proc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT = wnd_proc;
        SetWindowLongPtrW(hwnd, GWLP_WNDPROC, proc as usize as isize);
        DragAcceptFiles(hwnd, 1);
        eprintln!("[sys] 文件拖拽已注册（WndProc 子类化 + DragAcceptFiles）");
    }
}

/// 双击窗口（空白区域）：弹出系统原生文件选择对话框（非应用内窗口）。
fn open_file_dialog(audio: &AudioEngine, wave_tx: &Sender<PathBuf>) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("音频文件", &["mp3", "flac", "wav", "aac", "m4a", "ogg"])
        .pick_file()
    {
        enqueue_file(path, audio, wave_tx);
    }
}

fn main() {
    let ui = MainWindow::new().expect("创建窗口失败");
    let audio = Rc::new(AudioEngine::start());
    let (wave_tx, wave_rx) = spawn_waveform_worker();
    let (file_tx, file_rx) = mpsc::channel::<FileEvent>();
    let _ = FILE_EVENTS.set(file_tx.clone());

    // 初始音量 100%，与 UI 滑块保持一致。
    audio.send(Command::SetVolume(1.0));

    ui.show().expect("显示窗口失败");

    // winit 窗口是惰性创建的：事件循环启动（Resumed 阶段）后才真正存在，
    // 此前 `window_handle()` 返回 Unavailable。因此亚克力/圆角/拖拽注册等
    // 系统效果须等到窗口就绪后再应用（轮询检测，成功后停止）。
    let setup_timer = Rc::new(slint::Timer::default());
    {
        let ui = ui.clone_strong();
        let stop_handle = Rc::clone(&setup_timer);
        setup_timer.start(slint::TimerMode::Repeated, Duration::from_millis(50), move || {
            if hwnd_from_window(ui.window()).is_none() {
                return; // 窗口尚未创建，稍后重试。
            }
            stop_handle.stop();
            eprintln!("[sys] 窗口已创建，开始应用系统效果");
            apply_system_effects(ui.window());
            setup_drag_drop(ui.window());
        });
    }

    // 音量百分比提示的自动隐藏计时器。
    let volume_hide_timer = Rc::new(slint::Timer::default());

    // —— 回调接线 ——
    {
        let ui_weak = ui.as_weak();
        let audio = audio.clone();
        ui.global::<UIState>().on_toggle_play(move || {
            audio.send(Command::Toggle);
            // 本地同步播放状态，供播放/暂停按钮切换对应图标。
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                state.set_playing(!state.get_playing());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let audio = audio.clone();
        ui.global::<UIState>().on_seek_requested(move |fraction| {
            let Some(ui) = ui_weak.upgrade() else { return };
            let duration = ui.global::<UIState>().get_duration();
            audio.send(Command::Seek(Duration::from_secs_f32(fraction * duration)));
        });
    }
    {
        let ui_weak = ui.as_weak();
        let audio = audio.clone();
        // 快捷键左右方向键：相对当前播放位置快退/快进 5 秒。
        ui.global::<UIState>().on_seek_relative(move |delta| {
            let Some(ui) = ui_weak.upgrade() else { return };
            let state = ui.global::<UIState>();
            let duration = state.get_duration();
            let target = (state.get_position() + delta).clamp(
                0.0,
                if duration > 0.0 { duration } else { f32::MAX },
            );
            audio.send(Command::Seek(Duration::from_secs_f64(f64::from(target))));
        });
    }
    {
        let ui_weak = ui.as_weak();
        let audio = audio.clone();
        let volume_hide_timer = Rc::clone(&volume_hide_timer);
        ui.global::<UIState>().on_set_volume(move |volume| {
            let volume = volume.clamp(0.0, 1.0);
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                state.set_volume(volume);
                // 调整时显示音量百分比（1.5 秒后自动隐藏）。
                let percent = (volume * 100.0).round() as u32;
                state.set_volume_text(slint::SharedString::from(format!("{percent}%")));
                state.set_volume_showing(true);
            }
            audio.send(Command::SetVolume(volume));
            volume_hide_timer.restart();
        });
    }
    // 音量百分比自动隐藏计时器。
    {
        let ui_weak = ui.as_weak();
        let volume_hide_timer = Rc::clone(&volume_hide_timer);
        volume_hide_timer.start(
            slint::TimerMode::SingleShot,
            Duration::from_millis(1500),
            move || {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.global::<UIState>().set_volume_showing(false);
                }
            },
        );
    }
    {
        let ui_weak = ui.as_weak();
        // 右上角关闭按钮：隐藏窗口并退出事件循环。
        ui.global::<UIState>().on_close_window(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let _ = ui.window().hide();
            }
            let _ = slint::quit_event_loop();
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.global::<UIState>().on_toggle_pin(move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let state = ui.global::<UIState>();
            let on = !state.get_always_on_top();
            state.set_always_on_top(on);
            set_always_on_top(ui.window(), on);
        });
    }
    // 窗口拖动（空白区域按下 -> 跟随移动）+ 空白区域双击打开文件。
    // 用系统光标屏幕坐标计算位移，窗口移动不会再反过来干扰鼠标坐标，拖动跟手不闪烁。
    let drag_state = Rc::new(RefCell::new(None::<(slint::PhysicalPosition, i32, i32)>));
    let last_press = Rc::new(RefCell::new(None::<(Instant, f32, f32)>));
    {
        let ui_weak = ui.as_weak();
        let drag_state = Rc::clone(&drag_state);
        let last_press = Rc::clone(&last_press);
        let file_tx = file_tx.clone();
        ui.global::<UIState>().on_window_drag_down(move |x, y| {
            // 双击检测（窗口类无 CS_DBLCLKS，须自行判定）：两次按下
            // 间隔短且位置接近即视为双击。
            let now = Instant::now();
            let is_double = if let Some((t, px, py)) = *last_press.borrow() {
                now.duration_since(t) <= DOUBLE_CLICK_INTERVAL
                    && (x - px).abs() <= DOUBLE_CLICK_TOLERANCE
                    && (y - py).abs() <= DOUBLE_CLICK_TOLERANCE
            } else {
                false
            };
            *last_press.borrow_mut() = Some((now, x, y));
            if is_double {
                let _ = file_tx.send(FileEvent::DoubleClick);
            }
            if let Some(ui) = ui_weak.upgrade() {
                let origin = ui.window().position();
                match cursor_position() {
                    Some((cx, cy)) => {
                        *drag_state.borrow_mut() = Some((origin, cx, cy));
                    }
                    // 兜底：拿不到系统光标时用局部坐标近似。
                    None => {
                        let scale = ui.window().scale_factor();
                        *drag_state.borrow_mut() = Some((
                            origin,
                            (x * scale).round() as i32,
                            (y * scale).round() as i32,
                        ));
                    }
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let drag_state = Rc::clone(&drag_state);
        ui.global::<UIState>().on_window_drag_move(move |_, _| {
            let Some((origin, cx0, cy0)) = *drag_state.borrow() else { return };
            let Some(ui) = ui_weak.upgrade() else { return };
            let Some((cx, cy)) = cursor_position() else { return };
            ui.window().set_position(slint::PhysicalPosition::new(
                origin.x + (cx - cx0),
                origin.y + (cy - cy0),
            ));
        });
    }
    {
        let drag_state = Rc::clone(&drag_state);
        ui.global::<UIState>().on_window_drag_up(move || {
            *drag_state.borrow_mut() = None;
        });
    }

    // 启动参数（如“打开方式”传入的音乐文件）立即播放。
    for arg in std::env::args().skip(1) {
        let path = PathBuf::from(arg);
        if path.is_file() {
            enqueue_file(path, &audio, &wave_tx);
        }
    }

    // —— 周期性事件泵：音频事件 / 文件事件 / 波形结果 ——
    let timer = slint::Timer::default();
    // 波形按路径缓存：切到队列中下一首时立刻上屏，快速拖入多文件也不会错位。
    let mut waveform_cache: HashMap<PathBuf, WaveformResult> = HashMap::new();
    let mut cache_order: VecDeque<PathBuf> = VecDeque::new();
    let mut current_path: Option<PathBuf> = None;
    {
        let ui_weak = ui.as_weak();
        let audio = Rc::clone(&audio);
        let wave_tx = wave_tx.clone();
        timer.start(slint::TimerMode::Repeated, Duration::from_millis(100), move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let state = ui.global::<UIState>();

            while let Some(event) = audio.try_recv_event() {
                match event {
                    Event::TrackStarted { path } => {
                        current_path = Some(path.clone());
                        state.set_playing(true);
                        state.set_position(0.0);
                        if let Some(res) = waveform_cache.get(&path) {
                            apply_waveform(&state, res);
                        }
                        eprintln!("开始播放: {:?}", path);
                    }
                    Event::Position(pos) => state.set_position(pos.as_secs_f32()),
                    Event::Finished => {
                        state.set_playing(false);
                        state.set_position(state.get_duration());
                    }
                    Event::Error(e) => eprintln!("音频错误: {e}"),
                }
            }
            while let Ok(evt) = file_rx.try_recv() {
                match evt {
                    FileEvent::Dropped(paths) => {
                        for path in paths {
                            enqueue_file(path, &audio, &wave_tx);
                        }
                    }
                    FileEvent::DoubleClick => open_file_dialog(&audio, &wave_tx),
                }
            }
            while let Ok(res) = wave_rx.try_recv() {
                // 只把属于当前曲目的波形立即上屏；其余缓存，等切到该曲再显示。
                let is_current = current_path.as_ref().map_or(true, |p| *p == res.path);
                if !waveform_cache.contains_key(&res.path) {
                    cache_order.push_back(res.path.clone());
                    if cache_order.len() > WAVE_CACHE_LIMIT {
                        if let Some(oldest) = cache_order.pop_front() {
                            waveform_cache.remove(&oldest);
                        }
                    }
                }
                let path = res.path.clone();
                waveform_cache.insert(path.clone(), res);
                if is_current {
                    if let Some(cached) = waveform_cache.get(&path) {
                        apply_waveform(&state, cached);
                    }
                }
            }
        });
    }

    ui.run().expect("UI 事件循环失败");
}
