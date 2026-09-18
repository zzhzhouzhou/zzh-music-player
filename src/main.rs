//! zzhMusicPlayer —— 极简原生 Rust + Slint 桌面音乐播放器。
//! 长条形窗口 + Windows 11 亚克力毛玻璃 + 主题渐变背景。
//!
//! 本文件只是模块树根与程序入口：Slint 生成类型（MainWindow / TransportState /
//! PlaylistState / AboutState / PlaylistEntry / UpdateState）落在这里（crate 根），
//! 供各子模块经 crate:: 路径引用；全部装配逻辑在 app（组合根）。

// Release 构建下隐藏控制台窗口（纯 GUI 应用）。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod audio_engine;
mod events;
mod playlist;
mod pumps;
mod render_utils;
mod settings;
mod transport;
mod ui_callbacks;
mod updater;
mod version;
mod waveform;
mod waveform_cache;
mod waveform_generator;
mod windows_platform;

slint::include_modules!();

fn main() {
    app::run();
}
