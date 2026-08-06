//! zzhMusicPlayer —— 极简原生 Rust + Slint 桌面音乐播放器。
//! 长条形窗口 + Windows 11 亚克力毛玻璃 + 主题渐变背景。

// Release 构建下隐藏控制台窗口（纯 GUI 应用）。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_engine;
mod waveform_generator;

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use audio_engine::{AudioEngine, Command, Event, PlaybackMode};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use slint::ComponentHandle;
use slint::{Image, ModelRc, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};
use windows_sys::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{
    DwmSetWindowAttribute, DWMSBT_TRANSIENTWINDOW, DWMWA_SYSTEMBACKDROP_TYPE,
    DWMWA_USE_IMMERSIVE_DARK_MODE, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
};
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::System::Threading::CreateMutexW;
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, FindWindowW, GetCursorPos, GetWindowLongPtrW, MessageBoxW, SendMessageW,
    SetForegroundWindow, SetWindowLongPtrW, SetWindowPos, ShowWindow, SW_RESTORE, WM_CLOSE,
    WM_COPYDATA, WM_DROPFILES, WM_MOUSEWHEEL, GWLP_WNDPROC, HWND_NOTOPMOST, HWND_TOPMOST,
    MB_ICONWARNING, MB_OK, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
};

slint::include_modules!();

/// 双击判定的最大时间间隔（毫秒）。
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);
/// 双击判定的位置容差（逻辑像素）。
const DOUBLE_CLICK_TOLERANCE: f32 = 8.0;
/// 波形结果缓存上限：拖入大量文件时只保留最近若干份，避免内存无限增长。
/// 数值越小越省内存；切换回旧曲目时波形需重新生成。
const WAVE_CACHE_LIMIT: usize = 4;
/// 单例互斥体名（Local 前缀：互斥范围限当前登录会话）。
const SINGLE_INSTANCE_MUTEX: windows_sys::core::PCWSTR =
    windows_sys::core::w!("Local\\zzhMusicPlayer_SingleInstance");
/// WM_COPYDATA 自定义数据标识（转发“用本播放器打开”的文件路径列表）。
const WM_COPYDATA_OPEN_FILES: usize = 0x5A1E;
/// 等待已有实例窗口就绪的重试次数与间隔（窗口由 winit 惰性创建）。
const SINGLE_INSTANCE_RETRIES: u32 = 20;

/// 波形生成结果（后台线程产出，UI 线程消费；SharedPixelBuffer 为 Send）。
struct WaveformResult {
    path: PathBuf,
    bg: SharedPixelBuffer<Rgba8Pixel>,
    fg: SharedPixelBuffer<Rgba8Pixel>,
    duration: Duration,
    title: Option<String>,
    artist: Option<String>,
    theme: [u8; 3],
}

/// 文件相关外部事件（OS 拖拽 / 双击 / 滚轮 / 单例转发），经通道由 UI 线程统一处理。
enum FileEvent {
    Dropped(Vec<PathBuf>),
    /// 已运行实例通过 WM_COPYDATA 转发的“用本播放器打开”文件。
    OpenFiles(Vec<PathBuf>),
    DoubleClick,
    Wheel(i32),
    /// 关闭请求（右上角按钮或系统 WM_CLOSE）。
    CloseRequest,
}

/// WndProc 与 UI 线程之间的文件事件通道。
static FILE_EVENTS: OnceLock<Sender<FileEvent>> = OnceLock::new();
/// 被替换的原窗口过程（winit 的 WndProc）。
static ORIGINAL_WNDPROC: OnceLock<isize> = OnceLock::new();
/// 播放列表抽屉是否打开（打开时滚轮交给列表滚动，不调节音量）。
static PLAYLIST_OPEN: AtomicBool = AtomicBool::new(false);

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

/// 记忆设置：退出时保存，启动时恢复。
#[derive(Default)]
struct Settings {
    playlist: Vec<PathBuf>,
    position: f32,
    volume: f32,
    mode: PlaybackMode,
    pin: bool,
    current: Option<PathBuf>,
}

/// 设置文件路径：%APPDATA%\zzhMusicPlayer\settings.txt。
fn settings_path() -> PathBuf {
    let base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    base.join("zzhMusicPlayer").join("settings.txt")
}

fn load_settings() -> Settings {
    let mut s = Settings {
        volume: 1.0,
        mode: PlaybackMode::Sequential,
        ..Default::default()
    };
    let Ok(text) = std::fs::read_to_string(settings_path()) else {
        return s;
    };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else { continue };
        match key {
            "volume" => s.volume = value.parse().unwrap_or(1.0),
            "position" => s.position = value.parse().unwrap_or(0.0),
            "mode" => {
                s.mode = match value.parse::<u8>().unwrap_or(0) {
                    1 => PlaybackMode::ListLoop,
                    2 => PlaybackMode::SingleLoop,
                    3 => PlaybackMode::Random,
                    _ => PlaybackMode::Sequential,
                };
            }
            "pin" => s.pin = value == "1",
            "current" => s.current = Some(PathBuf::from(value)),
            "playlist" => s.playlist.push(PathBuf::from(value)),
            _ => {}
        }
    }
    s
}

fn save_settings(
    playlist: &[PathBuf],
    position: f32,
    volume: f32,
    mode: PlaybackMode,
    pin: bool,
    current: Option<&PathBuf>,
) {
    let path = settings_path();
    let _ = std::fs::create_dir_all(path.parent().unwrap_or(Path::new(".")));
    let mode = match mode {
        PlaybackMode::Sequential => 0,
        PlaybackMode::ListLoop => 1,
        PlaybackMode::SingleLoop => 2,
        PlaybackMode::Random => 3,
    };
    let mut out = String::new();
    out.push_str(&format!(
        "volume={volume}\nposition={position}\nmode={mode}\npin={}\n",
        u8::from(pin)
    ));
    if let Some(cur) = current {
        out.push_str(&format!("current={}\n", cur.display()));
    }
    for p in playlist {
        out.push_str(&format!("playlist={}\n", p.display()));
    }
    let _ = std::fs::write(path, out);
}

/// 播放列表显示名：文件名，缺失时用完整路径。
fn track_name(path: &Path) -> String {
    path.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// 启动波形后台线程：接收文件路径，解码生成波形位图、时长、主题色与响度。
fn spawn_waveform_worker() -> (Sender<PathBuf>, Receiver<WaveformResult>) {
    let (job_tx, job_rx) = mpsc::channel::<PathBuf>();
    let (res_tx, res_rx) = mpsc::channel::<WaveformResult>();
    std::thread::Builder::new()
        .name("waveform".to_string())
        .spawn(move || {
            while let Ok(path) = job_rx.recv() {
                let result = waveform_generator::analyze(&path).map(|wf| {
                    let (bg, fg) = waveform_generator::render_wave_buffers(&wf.columns, wf.theme);
                    WaveformResult {
                        path,
                        bg,
                        fg,
                        duration: wf.duration,
                        title: wf.title,
                        artist: wf.artist,
                        theme: wf.theme,
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

/// 颜色混合工具。
fn mix_rgb(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    [
        (f32::from(a[0]) + (f32::from(b[0]) - f32::from(a[0])) * t).round() as u8,
        (f32::from(a[1]) + (f32::from(b[1]) - f32::from(a[1])) * t).round() as u8,
        (f32::from(a[2]) + (f32::from(b[2]) - f32::from(a[2])) * t).round() as u8,
    ]
}

/// 生成柔和模糊感背景位图：低分辨率纵向渐变 + 若干主题色光斑，
/// 由 UI 平滑放大后呈现“高斯模糊”的柔和观感，体积极小（80×48）。
fn render_background(theme: [u8; 3]) -> SharedPixelBuffer<Rgba8Pixel> {
    const W: u32 = 80;
    const H: u32 = 48;
    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(W, H);
    let bytes = buf.make_mut_bytes();
    let stride = W as usize * 4;

    let black = [10u8, 12, 18];
    // 深色覆盖约 70%：颜色整体向深色收敛，仅保留主题色调。
    let top = mix_rgb(theme, black, 0.56);
    let bottom = mix_rgb(theme, black, 0.82);
    // (归一化x, 归一化y, 半径, 光斑色, 强度)：柔和光晕模拟高斯模糊，强度压低。
    let blobs: [(f32, f32, f32, [u8; 3], f32); 4] = [
        (0.24, 0.28, 0.46, mix_rgb(theme, black, 0.42), 0.14),
        (0.72, 0.18, 0.40, mix_rgb(theme, black, 0.50), 0.12),
        (0.52, 0.84, 0.48, mix_rgb(theme, black, 0.32), 0.10),
        (0.92, 0.62, 0.36, mix_rgb(theme, black, 0.46), 0.10),
    ];

    for y in 0..H {
        for x in 0..W {
            let fx = x as f32 / (W - 1) as f32;
            let fy = y as f32 / (H - 1) as f32;
            let mut r = f32::from(top[0]) + (f32::from(bottom[0]) - f32::from(top[0])) * fy;
            let mut g = f32::from(top[1]) + (f32::from(bottom[1]) - f32::from(top[1])) * fy;
            let mut b = f32::from(top[2]) + (f32::from(bottom[2]) - f32::from(top[2])) * fy;
            let a = 0.70 + fy * 0.10; // 暗色覆盖约 70%~80%，毛玻璃轻微透出。
            for &(bx, by, br, col, strength) in &blobs {
                let d = ((fx - bx).powi(2) + (fy - by).powi(2)).sqrt() / br;
                if d < 1.0 {
                    let f = (1.0 - d).powi(2) * strength;
                    r += (f32::from(col[0]) - r) * f;
                    g += (f32::from(col[1]) - g) * f;
                    b += (f32::from(col[2]) - b) * f;
                }
            }
            let i = y as usize * stride + x as usize * 4;
            bytes[i] = r.clamp(0.0, 255.0) as u8;
            bytes[i + 1] = g.clamp(0.0, 255.0) as u8;
            bytes[i + 2] = b.clamp(0.0, 255.0) as u8;
            bytes[i + 3] = (a.clamp(0.0, 1.0) * 255.0) as u8;
        }
    }
    buf
}

/// 把波形结果应用到 UI：波形图、时长、元数据与主题渐变背景。
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
    // 主题色 + 柔和模糊感背景。
    let theme = slint::Color::from_rgb_u8(res.theme[0], res.theme[1], res.theme[2]);
    state.set_theme_color(theme);
    state.set_bg_image(Image::from_rgba8(render_background(res.theme)));
}

/// 新文件加入播放列表：去重、同步模型与引擎、空闲时立即播放。
/// 返回新加入项的索引；已存在则返回 `None`。
fn add_track(
    path: PathBuf,
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    model: &Rc<VecModel<SharedString>>,
    state: &UIState,
    audio: &AudioEngine,
    wave_tx: &Sender<PathBuf>,
) -> Option<usize> {
    // 过滤非文件（不存在的路径 / 目录），静默跳过。
    if !path.is_file() {
        return None;
    }
    {
        let mut list = playlist.borrow_mut();
        if list.contains(&path) {
            return None;
        }
        list.push(path.clone());
        model.push(track_name(&path).into());
    }
    let idx = playlist.borrow().len() - 1;
    audio.send(Command::SetPlaylist(playlist.borrow().clone()));
    let _ = wave_tx.send(path);
    // 当前没有在播曲目时，新加入的文件立即开始播放。
    if state.get_playlist_current() < 0 {
        state.set_playlist_current(idx as i32);
        audio.send(Command::PlayAt(idx));
    }
    Some(idx)
}

/// 播放列表中的指定曲目。
fn play_at(
    index: usize,
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    state: &UIState,
    audio: &AudioEngine,
    wave_tx: &Sender<PathBuf>,
    waveform_cache: &Rc<RefCell<HashMap<PathBuf, WaveformResult>>>,
) {
    let list = playlist.borrow();
    if index >= list.len() {
        return;
    }
    state.set_playlist_current(index as i32);
    let path = list[index].clone();
    drop(list);
    audio.send(Command::PlayAt(index));
    // 缓存中没有才重新分析；已有则等 TrackStarted 直接上屏。
    if !waveform_cache.borrow().contains_key(&path) {
        let _ = wave_tx.send(path);
    }
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

/// 读取鼠标在屏幕上的物理坐标（用于平滑拖动窗口）。
fn cursor_position() -> Option<(i32, i32)> {
    let mut pt = POINT { x: 0, y: 0 };
    unsafe { (GetCursorPos(&mut pt) != 0).then_some((pt.x, pt.y)) }
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

/// 单例模式：创建命名互斥体。若已存在运行实例，把命令行文件转发给
/// 它的窗口（WM_COPYDATA）后退出本进程；首个实例则保持互斥体句柄。
fn enforce_single_instance() {
    unsafe {
        let mutex = CreateMutexW(std::ptr::null(), 1, SINGLE_INSTANCE_MUTEX);
        if mutex.is_null() {
            return; // 互斥体创建失败（罕见）：不阻塞正常启动。
        }
        if GetLastError() == ERROR_ALREADY_EXISTS {
            let files: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
            forward_to_running_instance(&files);
            std::process::exit(0);
        }
        // 首个实例：互斥体句柄由内核在进程退出时自动释放，此处仅保留变量防提前析构。
        let _ = mutex;
    }
}

/// 把文件路径列表经 WM_COPYDATA 发给已运行实例的窗口，并激活其前台显示。
fn forward_to_running_instance(files: &[PathBuf]) -> bool {
    unsafe {
        // winit 窗口惰性创建：轮询等待就绪（约 1 秒上限）。
        let mut hwnd: HWND = std::ptr::null_mut();
        for _ in 0..SINGLE_INSTANCE_RETRIES {
            hwnd = FindWindowW(std::ptr::null(), windows_sys::core::w!("zzhMusicPlayer"));
            if !hwnd.is_null() {
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        if hwnd.is_null() {
            // 极罕见竞态：窗口尚未就绪。提示用户，避免“打开方式”文件被静默丢弃。
            MessageBoxW(
                std::ptr::null_mut(),
                windows_sys::core::w!("无法连接到正在运行的播放器窗口，请稍后重试。"),
                windows_sys::core::w!("zzhMusicPlayer"),
                MB_ICONWARNING | MB_OK,
            );
            return false;
        }
        // 编码为 UTF-16 路径列表：每个路径以 \0 结尾，整体再以 \0 结尾。
        let mut data: Vec<u16> = Vec::new();
        for f in files {
            data.extend(f.to_string_lossy().encode_utf16());
            data.push(0);
        }
        data.push(0);
        let cd = COPYDATASTRUCT {
            dwData: WM_COPYDATA_OPEN_FILES,
            cbData: (data.len() * 2) as u32,
            lpData: data.as_mut_ptr() as *mut _,
        };
        SendMessageW(hwnd, WM_COPYDATA, 0, &cd as *const COPYDATASTRUCT as isize);
        ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd);
    }
    true
}

/// 把消息转发给原始窗口过程。
unsafe fn forward_to_original(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let original = ORIGINAL_WNDPROC.get().copied().unwrap_or(0);
    if original != 0 {
        let proc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT =
            unsafe { std::mem::transmute(original) };
        unsafe { proc(hwnd, msg, wparam, lparam) }
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
}

/// 子类化窗口过程：拦截文件拖拽、滚轮音量、WM_COPYDATA（单例转发）
/// 与系统关闭，其余消息转发原过程。
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DROPFILES => {
            let paths = unsafe { collect_dropped_files(wparam as *mut _) };
            if let Some(tx) = FILE_EVENTS.get() {
                let _ = tx.send(FileEvent::Dropped(paths));
            }
            0
        }
        WM_MOUSEWHEEL => {
            // 播放列表打开时交给列表滚动；否则滚轮调节音量。
            if PLAYLIST_OPEN.load(Ordering::Relaxed) {
                unsafe { forward_to_original(hwnd, msg, wparam, lparam) }
            } else {
                let delta = ((wparam >> 16) as u16 as i16) as i32;
                if let Some(tx) = FILE_EVENTS.get() {
                    let _ = tx.send(FileEvent::Wheel(delta));
                }
                0
            }
        }
        WM_CLOSE => {
            // 拦截系统关闭（Alt+F4 / 任务栏），统一走“保存设置再退出”。
            if let Some(tx) = FILE_EVENTS.get() {
                let _ = tx.send(FileEvent::CloseRequest);
            }
            0
        }
        WM_COPYDATA => {
            // 接收第二个实例转发的文件路径（UTF-16 列表，双重 \0 结尾）。
            let cd = lparam as *const COPYDATASTRUCT;
            if !cd.is_null() {
                let data = unsafe { &*cd };
                if data.dwData == WM_COPYDATA_OPEN_FILES && !data.lpData.is_null() {
                    // 上限 64KB，拒绝异常数据；逐元素非对齐读取（消息可来自任意进程）。
                    let len = (data.cbData as usize / 2).min(32 * 1024);
                    let base = data.lpData as *const u8;
                    let mut paths = Vec::new();
                    let mut cur = Vec::new();
                    for i in 0..len {
                        let u = unsafe { std::ptr::read_unaligned(base.add(i * 2) as *const u16) };
                        if u == 0 {
                            if !cur.is_empty() {
                                paths.push(PathBuf::from(String::from_utf16_lossy(&cur)));
                                cur.clear();
                            }
                        } else {
                            cur.push(u);
                        }
                    }
                    if !paths.is_empty()
                        && let Some(tx) = FILE_EVENTS.get()
                    {
                        let _ = tx.send(FileEvent::OpenFiles(paths));
                    }
                }
            }
            0
        }
        _ => unsafe { forward_to_original(hwnd, msg, wparam, lparam) },
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
fn open_file_dialog(
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    model: &Rc<VecModel<SharedString>>,
    state: &UIState,
    audio: &AudioEngine,
    wave_tx: &Sender<PathBuf>,
) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("音频文件", &["mp3", "flac", "wav", "aac", "m4a", "ogg"])
        .pick_file()
    {
        let _ = add_track(path, playlist, model, state, audio, wave_tx);
    }
}

/// 统一关闭流程：保存记忆设置、隐藏窗口并退出事件循环。
fn do_close(
    ui: &MainWindow,
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    mode_cell: &Rc<std::cell::Cell<PlaybackMode>>,
) {
    let state = ui.global::<UIState>();
    let current = {
        let list = playlist.borrow();
        let idx = state.get_playlist_current();
        if idx >= 0 {
            list.get(idx as usize).cloned()
        } else {
            None
        }
    };
    save_settings(
        &playlist.borrow(),
        state.get_position(),
        state.get_volume(),
        mode_cell.get(),
        state.get_always_on_top(),
        current.as_ref(),
    );
    let _ = ui.window().hide();
    let _ = slint::quit_event_loop();
}

fn main() {
    // 单例模式：已有实例时转发文件并退出，不创建第二个窗口。
    enforce_single_instance();

    let ui = MainWindow::new().expect("创建窗口失败");
    let audio = Rc::new(AudioEngine::start());
    let (wave_tx, wave_rx) = spawn_waveform_worker();
    let (file_tx, file_rx) = mpsc::channel::<FileEvent>();
    let _ = FILE_EVENTS.set(file_tx.clone());

    // —— 恢复记忆设置 ——
    let settings = load_settings();
    let state = ui.global::<UIState>();
    state.set_volume(settings.volume);
    state.set_volume_text(
        slint::SharedString::from(format!("{}%", (settings.volume * 100.0).round() as u32)),
    );
    state.set_mode_text(settings.mode.label().into());
    audio.send(Command::SetVolume(settings.volume));
    audio.send(Command::SetMode(settings.mode));

    // 播放列表（仅保留仍存在的文件，避免启动后大量报错）。
    let playlist: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));
    let playlist_model: Rc<VecModel<SharedString>> = Rc::new(VecModel::default());
    for path in &settings.playlist {
        if path.is_file() {
            playlist.borrow_mut().push(path.clone());
            playlist_model.push(track_name(path).into());
        }
    }
    state.set_playlist(ModelRc::from(playlist_model.clone()));
    audio.send(Command::SetPlaylist(playlist.borrow().clone()));

    let waveform_cache: Rc<RefCell<HashMap<PathBuf, WaveformResult>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let cache_order: Rc<RefCell<VecDeque<PathBuf>>> = Rc::new(RefCell::new(VecDeque::new()));
    let mode_cell: Rc<std::cell::Cell<PlaybackMode>> = Rc::new(std::cell::Cell::new(settings.mode));

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

    // 模式提示 / 音量弹层的自动隐藏计时器。
    let mode_hide_timer = Rc::new(slint::Timer::default());
    let popup_hide_timer = Rc::new(slint::Timer::default());
    // 粒子系统：每 33ms 推进相位，驱动白色粒子持续生成、飘散、淡出。
    let particle_timer = Rc::new(slint::Timer::default());
    {
        let ui_weak = ui.as_weak();
        particle_timer.start(slint::TimerMode::Repeated, Duration::from_millis(33), move || {
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                let t = state.get_particle_time() + 0.033;
                state.set_particle_time(if t >= 1.0 { t - 1.0 } else { t });
                // 工具栏悬停检测：光标进入工具栏矩形范围时让背景变实。
                if let Some((cx, cy)) = cursor_position() {
                    let scale = ui.window().scale_factor();
                    let origin = ui.window().position();
                    let x0 = origin.x + (210.0 * scale) as i32;
                    let x1 = origin.x + (510.0 * scale) as i32;
                    let y0 = origin.y + (120.0 * scale) as i32;
                    let y1 = origin.y + (160.0 * scale) as i32;
                    let hovered = cx >= x0 && cx <= x1 && cy >= y0 && cy <= y1;
                    state.set_toolbar_hovered(hovered);
                }
            }
        });
    }

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
        let audio = audio.clone();
        ui.global::<UIState>().on_next(move || audio.send(Command::Next));
    }
    {
        let audio = audio.clone();
        ui.global::<UIState>().on_previous(move || audio.send(Command::Prev));
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
    // 播放模式：顺序 → 列表循环 → 单曲循环 → 随机。
    {
        let ui_weak = ui.as_weak();
        let audio = audio.clone();
        let mode_cell = Rc::clone(&mode_cell);
        let mode_hide_timer = Rc::clone(&mode_hide_timer);
        ui.global::<UIState>().on_cycle_mode(move || {
            let mode = mode_cell.get().cycle();
            mode_cell.set(mode);
            audio.send(Command::SetMode(mode));
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                state.set_mode_text(mode.label().into());
                state.set_mode_showing(true);
            }
            mode_hide_timer.restart();
        });
    }
    {
        let ui_weak = ui.as_weak();
        let mode_hide_timer = Rc::clone(&mode_hide_timer);
        mode_hide_timer.start(
            slint::TimerMode::SingleShot,
            Duration::from_millis(1600),
            move || {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.global::<UIState>().set_mode_showing(false);
                }
            },
        );
    }
    // 音量：滑块 / 滚轮统一入口。
    {
        let ui_weak = ui.as_weak();
        let audio = audio.clone();
        let popup_hide_timer = Rc::clone(&popup_hide_timer);
        ui.global::<UIState>().on_set_volume(move |volume| {
            let volume = volume.clamp(0.0, 1.0);
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                state.set_volume(volume);
                state.set_volume_text(
                    slint::SharedString::from(format!("{}%", (volume * 100.0).round() as u32)),
                );
                // 调整音量时保持弹层可见，随后自动收起。
                state.set_volume_popup_open(true);
            }
            audio.send(Command::SetVolume(volume));
            popup_hide_timer.restart();
        });
    }
    // 音量弹层开关 + 自动收起。
    {
        let ui_weak = ui.as_weak();
        let popup_hide_timer = Rc::clone(&popup_hide_timer);
        ui.global::<UIState>().on_toggle_volume_popup(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                let open = !state.get_volume_popup_open();
                state.set_volume_popup_open(open);
                if open {
                    popup_hide_timer.restart();
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let popup_hide_timer = Rc::clone(&popup_hide_timer);
        popup_hide_timer.start(
            slint::TimerMode::SingleShot,
            Duration::from_millis(3000),
            move || {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.global::<UIState>().set_volume_popup_open(false);
                }
            },
        );
    }
    // 播放列表抽屉。
    {
        let ui_weak = ui.as_weak();
        ui.global::<UIState>().on_toggle_playlist(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                let open = !state.get_playlist_open();
                state.set_playlist_open(open);
                PLAYLIST_OPEN.store(open, Ordering::Relaxed);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let playlist = Rc::clone(&playlist);
        let audio = audio.clone();
        let wave_tx = wave_tx.clone();
        let waveform_cache = Rc::clone(&waveform_cache);
        ui.global::<UIState>().on_play_at(move |index| {
            let Some(ui) = ui_weak.upgrade() else { return };
            let state = ui.global::<UIState>();
            play_at(
                index as usize,
                &playlist,
                &state,
                &audio,
                &wave_tx,
                &waveform_cache,
            );
        });
    }
    {
        let ui_weak = ui.as_weak();
        let playlist = Rc::clone(&playlist);
        let model = Rc::clone(&playlist_model);
        let audio = audio.clone();
        ui.global::<UIState>().on_remove_track(move |index| {
            let index = index as usize;
            {
                let mut list = playlist.borrow_mut();
                if index >= list.len() {
                    return;
                }
                list.remove(index);
                model.remove(index);
            }
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                let cur = state.get_playlist_current();
                if cur as usize == index {
                    state.set_playlist_current(-1);
                } else if cur as usize > index {
                    state.set_playlist_current(cur - 1);
                }
                state.set_playlist(ModelRc::from(model.clone()));
            }
            // 引擎侧同步删除；若删的是当前播放曲目，引擎会自动切到下一首。
            audio.send(Command::RemoveAt(index));
        });
    }
    {
        let ui_weak = ui.as_weak();
        let playlist = Rc::clone(&playlist);
        let model = Rc::clone(&playlist_model);
        let audio = audio.clone();
        ui.global::<UIState>().on_clear_playlist(move || {
            playlist.borrow_mut().clear();
            model.set_vec(Vec::new());
            audio.send(Command::SetPlaylist(Vec::new()));
            if let Some(ui) = ui_weak.upgrade() {
                let state = ui.global::<UIState>();
                state.set_playlist_current(-1);
                state.set_playlist(ModelRc::from(model.clone()));
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let playlist = Rc::clone(&playlist);
        let mode_cell = Rc::clone(&mode_cell);
        ui.global::<UIState>().on_close_window(move || {
            if let Some(ui) = ui_weak.upgrade() {
                do_close(&ui, &playlist, &mode_cell);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.global::<UIState>().on_minimize_window(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.window().set_minimized(true);
            }
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

    // 恢复置顶状态与上次播放进度。
    if settings.pin {
        state.set_always_on_top(true);
        set_always_on_top(ui.window(), true);
    }
    if let Some(cur) = &settings.current
        && let Some(idx) = playlist.borrow().iter().position(|p| p == cur)
    {
        state.set_playlist_current(idx as i32);
        audio.send(Command::PlayAt(idx));
        let _ = wave_tx.send(cur.clone());
        if settings.position > 1.0 {
            audio.send(Command::Seek(Duration::from_secs_f32(settings.position)));
        }
    }

    // 启动参数（如“打开方式”传入的音乐文件）加入播放列表。
    for arg in std::env::args().skip(1) {
        let path = PathBuf::from(arg);
        if path.is_file() {
            let _ = add_track(
                path,
                &playlist,
                &playlist_model,
                &state,
                &audio,
                &wave_tx,
            );
        }
    }

    // —— 周期性事件泵：音频事件 / 文件事件 / 波形结果 ——
    let timer = slint::Timer::default();
    let mut current_path: Option<PathBuf> = None;
    {
        let ui_weak = ui.as_weak();
        let audio = Rc::clone(&audio);
        let wave_tx = wave_tx.clone();
        let playlist = Rc::clone(&playlist);
        let playlist_model = Rc::clone(&playlist_model);
        let waveform_cache = Rc::clone(&waveform_cache);
        let cache_order = Rc::clone(&cache_order);
        let popup_hide_timer = Rc::clone(&popup_hide_timer);
        let mode_cell = Rc::clone(&mode_cell);
        timer.start(slint::TimerMode::Repeated, Duration::from_millis(100), move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let state = ui.global::<UIState>();

            while let Some(event) = audio.try_recv_event() {
                match event {
                    Event::TrackStarted { path } => {
                        current_path = Some(path.clone());
                        state.set_playing(true);
                        state.set_position(0.0);
                        state.set_seek_pending(false);
                        state.set_dragging(false);
                        let idx = playlist.borrow().iter().position(|p| *p == path);
                        state.set_playlist_current(idx.map(|i| i as i32).unwrap_or(-1));
                        if let Some(res) = waveform_cache.borrow().get(&path) {
                            // 波形已就绪：完整上屏（标题/艺术家/波形图/背景/主题）。
                            apply_waveform(&state, res);
                        } else {
                            // 波形尚未生成：先用文件名即时上屏并清空上一首的波形/背景，
                            // 保证页面与音频同步，待波形线程完成后（wave_rx）再覆盖完整详情。
                            // 同时把路径交给后台线程生成波形——否则按钮切歌时永远不会生成。
                            let _ = wave_tx.send(path.clone());
                            let title = path
                                .file_stem()
                                .map(|s| s.to_string_lossy().into_owned())
                                .unwrap_or_default();
                            state.set_track_title(title.into());
                            state.set_track_artist(SharedString::default());
                            state.set_wave_bg_image(Image::default());
                            state.set_wave_fg_image(Image::default());
                            state.set_bg_image(Image::default());
                            state.set_duration(0.0);
                        }
                        eprintln!("开始播放: {:?}", path);
                    }
                    Event::Position(pos) => {
                        // 跳转确认：播放位置上报后释放预览，进度条直接落在目标处，不回弹。
                        if state.get_seek_pending() {
                            state.set_seek_pending(false);
                            state.set_dragging(false);
                        }
                        state.set_position(pos.as_secs_f32());
                    }
                    Event::Finished => {
                        state.set_playing(false);
                        state.set_position(state.get_duration());
                        state.set_seek_pending(false);
                        state.set_dragging(false);
                        current_path = None;
                        state.set_playlist_current(-1);
                    }
                    Event::Error(e) => {
                        // 跳转失败时同样解除预览，避免进度条卡在拖拽位置。
                        state.set_seek_pending(false);
                        state.set_dragging(false);
                        eprintln!("音频错误: {e}");
                    }
                }
            }
            while let Ok(evt) = file_rx.try_recv() {
                match evt {
                    FileEvent::Dropped(paths) => {
                        for path in paths {
                            let _ = add_track(
                                path,
                                &playlist,
                                &playlist_model,
                                &state,
                                &audio,
                                &wave_tx,
                            );
                        }
                    }
                    FileEvent::OpenFiles(paths) => {
                        // 第二个实例转发的“打开方式”文件：全部加入列表并立即播放首个新文件。
                        let mut first: Option<usize> = None;
                        for path in paths {
                            if let Some(idx) = add_track(
                                path,
                                &playlist,
                                &playlist_model,
                                &state,
                                &audio,
                                &wave_tx,
                            ) && first.is_none()
                            {
                                first = Some(idx);
                            }
                        }
                        if let Some(idx) = first {
                            play_at(
                                idx,
                                &playlist,
                                &state,
                                &audio,
                                &wave_tx,
                                &waveform_cache,
                            );
                        }
                    }
                    FileEvent::DoubleClick => open_file_dialog(
                        &playlist,
                        &playlist_model,
                        &state,
                        &audio,
                        &wave_tx,
                    ),
                    FileEvent::Wheel(delta) => {
                        let step = (delta as f32 / 120.0) * 0.05;
                        let volume = (state.get_volume() + step).clamp(0.0, 1.0);
                        state.set_volume(volume);
                        state.set_volume_text(
                            slint::SharedString::from(format!("{}%", (volume * 100.0).round() as u32)),
                        );
                        state.set_volume_popup_open(true);
                        audio.send(Command::SetVolume(volume));
                        popup_hide_timer.restart();
                    }
                    FileEvent::CloseRequest => {
                        // 保存设置并退出（拦截了系统 WM_CLOSE）。
                        let ui = ui_weak.upgrade();
                        if let Some(ui) = ui {
                            do_close(&ui, &playlist, &mode_cell);
                        }
                    }
                }
            }
            while let Ok(res) = wave_rx.try_recv() {
                // 只把属于当前曲目的波形立即上屏；其余缓存，等切到该曲再显示。
                let is_current = current_path.as_ref().is_some_and(|p| *p == res.path);
                {
                    let mut cache = waveform_cache.borrow_mut();
                    if !cache.contains_key(&res.path) {
                        cache_order.borrow_mut().push_back(res.path.clone());
                        if cache_order.borrow().len() > WAVE_CACHE_LIMIT
                            && let Some(oldest) = cache_order.borrow_mut().pop_front()
                        {
                            cache.remove(&oldest);
                        }
                    }
                    let path = res.path.clone();
                    cache.insert(path, res);
                }
                if is_current {
                    let cache = waveform_cache.borrow();
                    if let Some(cached) = cache.get(current_path.as_ref().unwrap().as_path()) {
                        apply_waveform(&state, cached);
                    }
                }
            }
        });
    }

    ui.run().expect("UI 事件循环失败");
}
