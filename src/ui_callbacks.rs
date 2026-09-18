//! Slint 全局回调接线：按领域分组注册（传输 / 播放列表 / 关于与更新 / 窗口壳层）。
//! 闭包只持有 Weak<App>，触发时升级访问共享状态；语义与拆分前逐一对应。

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use slint::{ComponentHandle, SharedString};

use crate::app::{App, close_playlist_window, do_close, open_playlist_window};
use crate::events::{FileEvent, UPDATE_INSTALLER_NAME, spawn_update_check};
use crate::playlist::{play_at, track_name};
use crate::windows_platform::{cursor_position, set_always_on_top, set_playlist_open};
use crate::{AboutState, PlaylistState, PlaylistWindow, TransportState, UpdateState};

/// 双击判定的最大时间间隔（毫秒）。
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);
/// 双击判定的位置容差（逻辑像素）。
const DOUBLE_CLICK_TOLERANCE: f32 = 8.0;

/// 注册全部 UI 回调。调用时机：App 组装完成、窗口 show 之后。
pub fn register_callbacks(app: &Rc<App>) {
    register_transport_callbacks(app);
    register_playlist_callbacks(app);
    register_about_callbacks(app);
    register_window_shell_callbacks(app);
}

/// 传输域：播放控制、跳转、模式、音量、窗口按钮。
fn register_transport_callbacks(app: &Rc<App>) {
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<TransportState>().on_toggle_play(move || {
            if let Some(app) = app_weak.upgrade() {
                app.audio.send(crate::audio_engine::Command::Toggle);
                // 本地同步播放状态，供播放/暂停按钮切换对应图标。
                let transport = app.ui.global::<TransportState>();
                transport.set_playing(!transport.get_playing());
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<TransportState>().on_next(move || {
            if let Some(app) = app_weak.upgrade() {
                app.audio.send(crate::audio_engine::Command::Next);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<TransportState>().on_previous(move || {
            if let Some(app) = app_weak.upgrade() {
                app.audio.send(crate::audio_engine::Command::Prev);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<TransportState>()
            .on_seek_requested(move |fraction| {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                let transport = app.ui.global::<TransportState>();
                let target = fraction * transport.get_duration();
                // 点击/拖拽跳转：锁定态，松手后显示立即钉在目标上，
                // 引擎尚未完成 seek 的旧上报由事件泵过滤。
                *app.seek_wait.borrow_mut() = Some(crate::transport::SeekState::Pending {
                    target,
                    since: Instant::now(),
                    lock: true,
                });
                transport.set_seek_lock_frac(fraction);
                transport.set_seek_lock(true);
                transport.set_position(target);
                transport.set_position_text(crate::render_utils::format_time(target));
                app.audio
                    .send(crate::audio_engine::Command::Seek(Duration::from_secs_f32(
                        target,
                    )));
            });
    }
    {
        let app_weak = Rc::downgrade(app);
        // 快捷键左右方向键：相对当前播放位置快退/快进 5 秒。
        app.ui
            .global::<TransportState>()
            .on_seek_relative(move |delta| {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                let transport = app.ui.global::<TransportState>();
                let duration = transport.get_duration();
                let target = (transport.get_position() + delta)
                    .clamp(0.0, if duration > 0.0 { duration } else { f32::MAX });
                // 方向键快进快退：不进入锁定态，位置属性直接更新到目标，
                // 由 played-frac 的 200ms 插值动画平滑滑过去；陈旧位置事件
                // 仍由 seek_wait 过滤（未锁定时只顶替数值，不钉显示）。
                *app.seek_wait.borrow_mut() = Some(crate::transport::SeekState::Pending {
                    target,
                    since: Instant::now(),
                    lock: false,
                });
                transport.set_position(target);
                transport.set_position_text(crate::render_utils::format_time(target));
                app.audio
                    .send(crate::audio_engine::Command::Seek(Duration::from_secs_f64(
                        f64::from(target),
                    )));
            });
    }
    // 播放模式：顺序 → 列表循环 → 单曲循环 → 随机。
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<TransportState>().on_cycle_mode(move || {
            if let Some(app) = app_weak.upgrade() {
                let mode = app.mode_cell.get().cycle();
                app.mode_cell.set(mode);
                app.audio.send(crate::audio_engine::Command::SetMode(mode));
                let transport = app.ui.global::<TransportState>();
                transport.set_mode_text(mode.label().into());
                transport.set_mode_showing(true);
                app.mode_hide_timer.restart();
            }
        });
    }
    // 音量：滑块 / 滚轮统一入口。
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<TransportState>()
            .on_set_volume(move |volume| {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                let volume = volume.clamp(0.0, 1.0);
                let transport = app.ui.global::<TransportState>();
                transport.set_volume(volume);
                transport.set_volume_text(SharedString::from(format!(
                    "{}%",
                    (volume * 100.0).round() as u32
                )));
                // 调整音量时保持弹层可见，随后自动收起。
                transport.set_volume_popup_open(true);
                app.audio
                    .send(crate::audio_engine::Command::SetVolume(volume));
                app.popup_hide_timer.restart();
            });
    }
    // 音量弹层开关 + 自动收起。
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<TransportState>()
            .on_toggle_volume_popup(move || {
                if let Some(app) = app_weak.upgrade() {
                    let transport = app.ui.global::<TransportState>();
                    let open = !transport.get_volume_popup_open();
                    transport.set_volume_popup_open(open);
                    if open {
                        app.popup_hide_timer.restart();
                    }
                }
            });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<TransportState>().on_close_window(move || {
            if let Some(app) = app_weak.upgrade() {
                do_close(&app);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<TransportState>()
            .on_minimize_window(move || {
                if let Some(app) = app_weak.upgrade() {
                    app.ui.window().set_minimized(true);
                }
            });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<TransportState>().on_toggle_pin(move || {
            if let Some(app) = app_weak.upgrade() {
                let transport = app.ui.global::<TransportState>();
                let on = !transport.get_always_on_top();
                transport.set_always_on_top(on);
                set_always_on_top(app.ui.window(), on);
            }
        });
    }
}

/// 播放列表域：抽屉开关、点歌、搜索、拖拽排序、删除、清空、打开文件夹。
/// 动作实现集中在下方 action_* 共享函数：主窗口抽屉与独立弹窗的
/// Slint 全局互不相通（按组件实例隔离），但 Rust 侧共享状态是唯一事实源。
fn register_playlist_callbacks(app: &Rc<App>) {
    // 播放列表抽屉开关（主控按钮）。
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<PlaylistState>()
            .on_toggle_playlist(move || {
                if let Some(app) = app_weak.upgrade() {
                    // 已弹出为独立窗口：主控按钮此时负责收回弹窗。
                    if app.playlist_window.borrow().is_some() {
                        close_playlist_window(&app);
                        return;
                    }
                    let playlist_state = app.ui.global::<PlaylistState>();
                    let open = !playlist_state.get_playlist_open();
                    playlist_state.set_playlist_open(open);
                    set_playlist_open(open);
                    if !open {
                        // 收起抽屉时一并清掉搜索过滤，下次展开是完整列表。
                        playlist_state.set_search_open(false);
                        playlist_state.set_search_text(SharedString::default());
                        app.playlist_view
                            .borrow_mut()
                            .set_filter("", &app.playlist.borrow());
                    }
                }
            });
    }
    // 弹出为独立窗口（阶段 C）：收起抽屉并复位过滤，内容转由弹窗承载。
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<PlaylistState>().on_pop_out(move || {
            if let Some(app) = app_weak.upgrade() {
                let playlist_state = app.ui.global::<PlaylistState>();
                playlist_state.set_playlist_open(false);
                playlist_state.set_search_open(false);
                playlist_state.set_search_text(SharedString::default());
                app.playlist_view
                    .borrow_mut()
                    .set_filter("", &app.playlist.borrow());
                open_playlist_window(&app);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<PlaylistState>().on_play_at(move |index| {
            if let Some(app) = app_weak.upgrade() {
                action_play_at(&app, index);
            }
        });
    }
    // 拖动开始时按显示行号取歌名填充浮块（Slint 不支持动态模型下标）。
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<PlaylistState>()
            .on_set_reorder_text(move |row| {
                if let Some(app) = app_weak.upgrade()
                    && let Some(name) = action_reorder_text(&app, row)
                {
                    app.ui
                        .global::<PlaylistState>()
                        .set_reorder_text(name.into());
                }
            });
    }
    // 搜索框文本变化：重建过滤后的显示模型。
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<PlaylistState>()
            .on_search_edited(move |text| {
                if let Some(app) = app_weak.upgrade() {
                    action_search_edited(&app, &text);
                }
            });
    }
    // 列表拖动排序：把 from 行移动到 to 位置（Slint 传来 float，此处取整钳制）。
    // 搜索过滤中 UI 已禁用起拖，这里再兜底拒绝，防止行号错位。
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<PlaylistState>()
            .on_move_track(move |from, to| {
                if let Some(app) = app_weak.upgrade() {
                    action_move_track(&app, from, to);
                }
            });
    }
    // 在资源管理器中打开曲目所在文件夹并选中文件。
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<PlaylistState>()
            .on_open_folder(move |index| {
                if let Some(app) = app_weak.upgrade() {
                    action_open_folder(&app, index);
                }
            });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui
            .global::<PlaylistState>()
            .on_remove_track(move |index| {
                if let Some(app) = app_weak.upgrade() {
                    action_remove_track(&app, index);
                }
            });
    }
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<PlaylistState>().on_clear_playlist(move || {
            if let Some(app) = app_weak.upgrade() {
                action_clear_playlist(&app);
            }
        });
    }
}

// —— 播放列表动作的共享实现：主窗口抽屉与独立弹窗的回调都落到这里 ——
// playlist_current 等写入主窗口实例的属性由事件泵同步到弹窗（pumps.rs），
// 因此这些函数统一经 app.ui 访问全局即可；需要“写回发起窗口自身”的
// 返回值（如浮块文本）由调用方处理。

fn action_play_at(app: &App, index: f32) {
    let playlist_state = app.ui.global::<PlaylistState>();
    // 显示行号 → 真实索引（搜索过滤后两者不一致）。
    let index = app
        .playlist_view
        .borrow()
        .real_of(index.round().max(0.0) as usize);
    play_at(index, &app.playlist, &playlist_state, &app.audio);
}

fn action_remove_track(app: &App, index: f32) {
    let display = index.round().max(0.0) as usize;
    let real = app.playlist_view.borrow().real_of(display);
    {
        let mut list = app.playlist.borrow_mut();
        if real >= list.len() {
            return;
        }
        list.remove(real);
    }
    app.playlist_view
        .borrow_mut()
        .removed(&app.playlist.borrow());
    let playlist_state = app.ui.global::<PlaylistState>();
    let cur = playlist_state.get_playlist_current();
    if cur as usize == real {
        playlist_state.set_playlist_current(-1);
    } else if cur as usize > real {
        playlist_state.set_playlist_current(cur - 1);
    }
    // 引擎侧同步删除；若删的是当前播放曲目，引擎会自动切到下一首。
    app.audio.send(crate::audio_engine::Command::RemoveAt(real));
}

fn action_move_track(app: &App, from: f32, to: f32) {
    let playlist_state = app.ui.global::<PlaylistState>();
    let mut view = app.playlist_view.borrow_mut();
    if view.is_filtering() {
        return;
    }
    let len = app.playlist.borrow().len();
    let from = from.round().max(0.0) as usize;
    let to = (to.round().max(0.0) as usize).min(len.saturating_sub(1));
    if from >= len || from == to {
        return;
    }
    let item = app.playlist.borrow_mut().remove(from);
    app.playlist.borrow_mut().insert(to, item);
    view.moved(&app.playlist.borrow());
    drop(view);
    // 当前曲目索引随移动平移（引擎侧按路径重定位，无需单独命令）。
    let cur = playlist_state.get_playlist_current() as i64;
    let (f, t) = (from as i64, to as i64);
    let new_cur = if cur == f {
        t
    } else if f < cur && cur <= t {
        cur - 1
    } else if t <= cur && cur < f {
        cur + 1
    } else {
        cur
    };
    playlist_state.set_playlist_current(new_cur as i32);
    app.audio.send(crate::audio_engine::Command::SetPlaylist(
        app.playlist.borrow().clone(),
    ));
}

#[cfg(windows)]
fn action_open_folder(app: &App, index: f32) {
    use std::os::windows::process::CommandExt;
    let index = app
        .playlist_view
        .borrow()
        .real_of(index.round().max(0.0) as usize);
    if let Some(p) = app.playlist.borrow().get(index) {
        // explorer /select,"路径"：打开文件夹并高亮该文件。
        let _ = std::process::Command::new("explorer.exe")
            .raw_arg(format!("/select,\"{}\"", p.display()))
            .spawn();
    }
}

#[cfg(not(windows))]
fn action_open_folder(app: &App, index: f32) {
    let _ = (app, index);
}

fn action_search_edited(app: &App, text: &str) {
    app.playlist_view
        .borrow_mut()
        .set_filter(text, &app.playlist.borrow());
}

fn action_clear_playlist(app: &App) {
    app.playlist.borrow_mut().clear();
    app.playlist_view.borrow_mut().cleared();
    app.audio
        .send(crate::audio_engine::Command::SetPlaylist(Vec::new()));
    let playlist_state = app.ui.global::<PlaylistState>();
    playlist_state.set_playlist_current(-1);
}

/// 按显示行号取歌名（填充拖动浮块）；行号越界返回 None。
fn action_reorder_text(app: &App, row: f32) -> Option<String> {
    let row = app
        .playlist_view
        .borrow()
        .real_of(row.round().max(0.0) as usize);
    app.playlist.borrow().get(row).map(|p| track_name(p))
}

/// 独立播放列表窗口的回调接线：与主窗口抽屉共用同一批动作实现，但
/// “写回发起窗口自身属性”的回调（浮块文本）必须落到本窗口的全局实例，
/// 因此这些回调经 pw_weak 升级后取本窗口的 global，避免持有强引用自环。
pub(crate) fn register_playlist_window_callbacks(app: &Rc<App>, pw: &PlaylistWindow) {
    let ps = pw.global::<PlaylistState>();
    let ts = pw.global::<TransportState>();
    let pw_weak = pw.as_weak();

    {
        let app_weak = Rc::downgrade(app);
        ps.on_play_at(move |index| {
            if let Some(app) = app_weak.upgrade() {
                action_play_at(&app, index);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        ps.on_remove_track(move |index| {
            if let Some(app) = app_weak.upgrade() {
                action_remove_track(&app, index);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        ps.on_move_track(move |from, to| {
            if let Some(app) = app_weak.upgrade() {
                action_move_track(&app, from, to);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        ps.on_open_folder(move |index| {
            if let Some(app) = app_weak.upgrade() {
                action_open_folder(&app, index);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        ps.on_search_edited(move |text| {
            if let Some(app) = app_weak.upgrade() {
                action_search_edited(&app, &text);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        ps.on_clear_playlist(move || {
            if let Some(app) = app_weak.upgrade() {
                action_clear_playlist(&app);
            }
        });
    }
    {
        let app_weak = Rc::downgrade(app);
        let pw_weak = pw_weak.clone();
        ps.on_set_reorder_text(move |row| {
            if let (Some(app), Some(pw)) = (app_weak.upgrade(), pw_weak.upgrade())
                && let Some(name) = action_reorder_text(&app, row)
            {
                pw.global::<PlaylistState>().set_reorder_text(name.into());
            }
        });
    }
    // 弹窗内关闭按钮（面板 close-requested → pop-close）。
    {
        let app_weak = Rc::downgrade(app);
        ps.on_pop_close(move || {
            if let Some(app) = app_weak.upgrade() {
                close_playlist_window(&app);
            }
        });
    }

    // —— 标题区拖动本窗口（与主窗口同一套光标跟随，无双击打开文件语义）——
    let drag_state: Rc<RefCell<Option<(slint::PhysicalPosition, i32, i32)>>> =
        Rc::new(RefCell::new(None));
    {
        let pw_weak = pw_weak.clone();
        let drag_state = Rc::clone(&drag_state);
        ts.on_window_drag_down(move |_, _| {
            let Some(pw) = pw_weak.upgrade() else {
                return;
            };
            let Some((cx, cy)) = cursor_position() else {
                return;
            };
            *drag_state.borrow_mut() = Some((pw.window().position(), cx, cy));
        });
    }
    {
        let pw_weak = pw_weak.clone();
        let drag_state = Rc::clone(&drag_state);
        ts.on_window_drag_move(move |_, _| {
            let Some((origin, cx0, cy0)) = *drag_state.borrow() else {
                return;
            };
            let Some(pw) = pw_weak.upgrade() else {
                return;
            };
            let Some((cx, cy)) = cursor_position() else {
                return;
            };
            pw.window().set_position(slint::PhysicalPosition::new(
                origin.x + (cx - cx0),
                origin.y + (cy - cy0),
            ));
        });
    }
    {
        let drag_state = Rc::clone(&drag_state);
        ts.on_window_drag_up(move || {
            *drag_state.borrow_mut() = None;
        });
    }
}

/// 关于与更新域：检查更新、安装重启、打开仓库主页。
fn register_about_callbacks(app: &Rc<App>) {
    // 检查更新：手动触发时确认有新版即自动下载（启动探测不自动下载）。
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<AboutState>().on_check_updates(move || {
            if let Some(app) = app_weak.upgrade() {
                let about_state = app.ui.global::<AboutState>();
                // 下载中 / 已就绪 / 检查中不重复触发。
                if about_state.get_update_state() == UpdateState::Downloading
                    || about_state.get_update_state() == UpdateState::Ready
                    || about_state.get_update_state() == UpdateState::Checking
                {
                    return;
                }
                spawn_update_check(app.update_tx.clone(), true);
            }
        });
    }
    // 安装并重启：拉起分离进程（延迟 2 秒 → 静默安装 → 自动重启新版本），
    // 本程序随即保存设置退出，安装器不会遇到文件占用。
    {
        let app_weak = Rc::downgrade(app);
        app.ui.global::<AboutState>().on_install_update(move || {
            let Some(app) = app_weak.upgrade() else { return };
            let installer = std::env::temp_dir().join(UPDATE_INSTALLER_NAME);
            if !installer.is_file() {
                let about_state = app.ui.global::<AboutState>();
                about_state.set_update_state(UpdateState::Failed);
                about_state.set_update_note("安装包丢失，请重新检查更新".into());
                return;
            }
            let Ok(exe) = std::env::current_exe() else { return };
            // 写临时 .cmd 脚本再拉起：延迟 2 秒（等本程序完全退出）→ 静默安装
            // → 自动重启新版本。脚本自删除；.cmd 规避 cmd /C 长命令的引号转义问题。
            let script_path = std::env::temp_dir().join("zzh_update_launch.cmd");
            let script = format!(
                "@echo off\r\nping -n 3 127.0.0.1 >nul\r\nstart \"\" /wait \"{}\" /VERYSILENT /SUPPRESSMSGBOXES /NORESTART\r\nstart \"\" \"{}\"\r\ndel \"%~f0\"\r\n",
                installer.display(),
                exe.display()
            );
            if std::fs::write(&script_path, script).is_err() {
                let about_state = app.ui.global::<AboutState>();
                about_state.set_update_state(UpdateState::Failed);
                about_state.set_update_note("无法创建安装脚本".into());
                return;
            }
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                // CREATE_NO_WINDOW：不闪黑框。
                let _ = std::process::Command::new("cmd")
                    .args(["/C", &script_path.display().to_string()])
                    .creation_flags(0x0800_0000)
                    .spawn();
            }
            do_close(&app);
        });
    }
    // “关于”里的 GitHub 图标：跳转到项目仓库。
    {
        app.ui.global::<AboutState>().on_open_github(move || {
            let _ = std::process::Command::new("rundll32")
                .args([
                    "url.dll,FileProtocolHandler",
                    "https://github.com/zzhzhouzhou/zzh-music-player",
                ])
                .spawn();
        });
    }
}

/// 窗口壳层：空白区域拖动、双击打开文件。
fn register_window_shell_callbacks(app: &Rc<App>) {
    let drag_state: Rc<RefCell<Option<(slint::PhysicalPosition, i32, i32)>>> =
        Rc::new(RefCell::new(None));
    let last_press: Rc<RefCell<Option<(Instant, f32, f32)>>> = Rc::new(RefCell::new(None));
    // 窗口拖动（空白区域按下 -> 跟随移动）+ 空白区域双击打开文件。
    {
        let app_weak = Rc::downgrade(app);
        let drag_state = Rc::clone(&drag_state);
        let last_press = Rc::clone(&last_press);
        app.ui
            .global::<TransportState>()
            .on_window_drag_down(move |x, y| {
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
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
                    let _ = app.file_tx.send(FileEvent::DoubleClick);
                }
                let origin = app.ui.window().position();
                match cursor_position() {
                    Some((cx, cy)) => {
                        *drag_state.borrow_mut() = Some((origin, cx, cy));
                    }
                    // 兜底：拿不到系统光标时用局部坐标近似。
                    None => {
                        let scale = app.ui.window().scale_factor();
                        *drag_state.borrow_mut() = Some((
                            origin,
                            (x * scale).round() as i32,
                            (y * scale).round() as i32,
                        ));
                    }
                }
            });
    }
    {
        let app_weak = Rc::downgrade(app);
        let drag_state = Rc::clone(&drag_state);
        app.ui
            .global::<TransportState>()
            .on_window_drag_move(move |_, _| {
                let Some((origin, cx0, cy0)) = *drag_state.borrow() else {
                    return;
                };
                let Some(app) = app_weak.upgrade() else {
                    return;
                };
                let Some((cx, cy)) = cursor_position() else {
                    return;
                };
                app.ui.window().set_position(slint::PhysicalPosition::new(
                    origin.x + (cx - cx0),
                    origin.y + (cy - cy0),
                ));
            });
    }
    {
        let drag_state = Rc::clone(&drag_state);
        app.ui
            .global::<TransportState>()
            .on_window_drag_up(move || {
                *drag_state.borrow_mut() = None;
            });
    }
}
