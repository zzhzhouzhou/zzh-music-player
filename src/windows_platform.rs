//! Windows 窗口效果、单实例通信与原生文件事件集成。

use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::time::Duration;

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows_sys::Win32::Foundation::{
    ERROR_ALREADY_EXISTS, GetLastError, HWND, LPARAM, LRESULT, POINT, WPARAM,
};
use windows_sys::Win32::Graphics::Dwm::{
    DWMSBT_TRANSIENTWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_USE_IMMERSIVE_DARK_MODE,
    DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND, DwmSetWindowAttribute,
};
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::System::Threading::CreateMutexW;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, FindWindowW, GWLP_WNDPROC, GetCursorPos, GetWindowLongPtrW, HTCAPTION,
    HWND_NOTOPMOST, HWND_TOPMOST, MB_ICONWARNING, MB_OK, MessageBoxW, SW_RESTORE, SWP_NOACTIVATE,
    SWP_NOMOVE, SWP_NOSIZE, SendMessageW, SetForegroundWindow, SetWindowLongPtrW, SetWindowPos,
    ShowWindow, WM_CLOSE, WM_COPYDATA, WM_DROPFILES, WM_MOUSEWHEEL, WM_NCLBUTTONDOWN,
};

use crate::events::FileEvent;

const SINGLE_INSTANCE_MUTEX: windows_sys::core::PCWSTR =
    windows_sys::core::w!("Local\\zzhMusicPlayer_SingleInstance");
/// WM_COPYDATA 自定义数据标识（转发“用本播放器打开”的文件路径列表）。
const WM_COPYDATA_OPEN_FILES: usize = 0x5A1E;
/// 等待已有实例窗口就绪的重试次数与间隔（窗口由 winit 惰性创建）。
const SINGLE_INSTANCE_RETRIES: u32 = 20;

/// WndProc 与 UI 线程之间的文件事件通道。
static FILE_EVENTS: OnceLock<Sender<FileEvent>> = OnceLock::new();
/// 被替换的原窗口过程（winit 的 WndProc）。
static ORIGINAL_WNDPROC: OnceLock<isize> = OnceLock::new();
/// 播放列表独立窗口被替换的原窗口过程（与主窗口分开保存，互不覆盖）。
static POPOUT_ORIGINAL_WNDPROC: OnceLock<isize> = OnceLock::new();
/// 播放列表抽屉是否打开（打开时滚轮交给列表滚动，不调节音量）。
static PLAYLIST_OPEN: AtomicBool = AtomicBool::new(false);
/// “关于”对话框是否打开（打开时滚轮不再调整音量）。
static ABOUT_OPEN: AtomicBool = AtomicBool::new(false);

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

/// 从 `slint::Window` 获取原生 HWND。
pub(crate) fn hwnd_from_window(window: &slint::Window) -> Option<HWND> {
    let handle = window.window_handle();
    let rwh = handle.window_handle().ok()?;
    match rwh.as_raw() {
        RawWindowHandle::Win32(win32) => Some(win32.hwnd.get() as *mut _),
        _ => None,
    }
}

/// 读取鼠标在屏幕上的物理坐标（用于平滑拖动窗口）。
pub(crate) fn cursor_position() -> Option<(i32, i32)> {
    let mut pt = POINT { x: 0, y: 0 };
    unsafe { (GetCursorPos(&mut pt) != 0).then_some((pt.x, pt.y)) }
}

/// 应用 Windows 11 亚克力毛玻璃、深色着色与圆角。
///
/// 优先使用 DWM 系统背景（Win11 22H2+，`DWMSBT_TRANSIENTWINDOW` 即 Acrylic），
/// 失败则降级 `SetWindowCompositionAttribute`（Win10 20H1+ / Win11 全版本）。
pub(crate) fn apply_system_effects(window: &slint::Window) {
    let Some(hwnd) = hwnd_from_window(window) else {
        return;
    };
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
        let Some(proc) = GetProcAddress(
            module,
            c"SetWindowCompositionAttribute".as_ptr() as *const u8,
        ) else {
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
pub(crate) fn set_always_on_top(window: &slint::Window, on: bool) {
    let Some(hwnd) = hwnd_from_window(window) else {
        return;
    };
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
            paths.push(PathBuf::from(String::from_utf16_lossy(
                &buf[..len as usize],
            )));
        }
        DragFinish(drop);
    }
    paths
}

/// 单例模式：创建命名互斥体。若已存在运行实例，把命令行文件转发给
/// 它的窗口（WM_COPYDATA）后退出本进程；首个实例则保持互斥体句柄。
pub(crate) fn enforce_single_instance() {
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

/// 把消息转发给指定的原窗口过程。
unsafe fn forward_to(
    original: isize,
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if original != 0 {
        let proc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT =
            unsafe { std::mem::transmute(original) };
        unsafe { proc(hwnd, msg, wparam, lparam) }
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
}

/// 把消息转发给主窗口的原窗口过程。
unsafe fn forward_to_original(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        forward_to(
            ORIGINAL_WNDPROC.get().copied().unwrap_or(0),
            hwnd,
            msg,
            wparam,
            lparam,
        )
    }
}

/// 把消息转发给播放列表独立窗口的原窗口过程。
unsafe fn forward_to_popout_original(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        forward_to(
            POPOUT_ORIGINAL_WNDPROC.get().copied().unwrap_or(0),
            hwnd,
            msg,
            wparam,
            lparam,
        )
    }
}

/// 子类化窗口过程：拦截文件拖拽、滚轮音量、WM_COPYDATA（单例转发）
/// 与系统关闭，其余消息转发原过程。
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DROPFILES => {
            let paths = unsafe { collect_dropped_files(wparam as *mut _) };
            if let Some(tx) = FILE_EVENTS.get() {
                let _ = tx.send(FileEvent::Dropped(paths));
            }
            0
        }
        WM_MOUSEWHEEL => {
            // 播放列表打开时交给列表滚动；“关于”打开时不响应（防误调音量）；
            // 否则滚轮调节音量。
            if ABOUT_OPEN.load(Ordering::Relaxed) {
                0
            } else if PLAYLIST_OPEN.load(Ordering::Relaxed) {
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
pub(crate) fn setup_drag_drop(window: &slint::Window) {
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

/// 进入系统原生窗口拖动（无边框窗口的标准做法）：释放鼠标捕获后向本窗口
/// 发送 WM_NCLBUTTONDOWN/HTCAPTION，由 OS 模态循环接管拖动直至松开按键。
/// 不自己用 GetCursorPos 增量算位置——系统光标读数与 Slint 的坐标空间在
/// 混合 DPI / 远程会话下存在漂移偏移，自算会让窗口跑离光标导致拖动中断。
pub(crate) fn begin_native_drag(window: &slint::Window) {
    let Some(hwnd) = hwnd_from_window(window) else {
        return;
    };
    unsafe {
        ReleaseCapture();
        SendMessageW(hwnd, WM_NCLBUTTONDOWN, HTCAPTION as usize, 0);
    }
}

/// 子类化播放列表独立窗口：仅拦 WM_CLOSE（Alt+F4 收回弹窗而非退出程序）。
/// 不注册文件拖拽（拖拽入口保留在主窗口）；滚轮等其余消息原样转发给
/// winit，由 Slint 路由到弹窗内元素（列表滚轮在面板覆盖层处理）。
pub(crate) fn setup_playlist_window(window: &slint::Window) {
    let Some(hwnd) = hwnd_from_window(window) else {
        eprintln!("[sys] 弹窗子类化失败：获取 HWND 失败");
        return;
    };
    unsafe {
        let original = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
        if original == 0 {
            eprintln!("[sys] 弹窗子类化失败：获取原 WndProc 失败");
            return;
        }
        let _ = POPOUT_ORIGINAL_WNDPROC.set(original);
        let proc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT = popout_wnd_proc;
        SetWindowLongPtrW(hwnd, GWLP_WNDPROC, proc as usize as isize);
    }
}

/// 播放列表独立窗口的窗口过程：只拦 WM_CLOSE，其余全部转发。
unsafe extern "system" fn popout_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            if let Some(tx) = FILE_EVENTS.get() {
                let _ = tx.send(FileEvent::PlaylistWindowClose);
            }
            0
        }
        _ => unsafe { forward_to_popout_original(hwnd, msg, wparam, lparam) },
    }
}

pub(crate) fn set_file_events(sender: Sender<FileEvent>) {
    let _ = FILE_EVENTS.set(sender);
}

pub(crate) fn set_playlist_open(open: bool) {
    PLAYLIST_OPEN.store(open, Ordering::Relaxed);
}

pub(crate) fn set_about_open(open: bool) {
    ABOUT_OPEN.store(open, Ordering::Relaxed);
}

pub(crate) fn is_about_open() -> bool {
    ABOUT_OPEN.load(Ordering::Relaxed)
}
