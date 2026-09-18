use std::path::{Path, PathBuf};

use crate::audio_engine::{EqSettings, PlaybackMode};

/// 记忆设置：退出时保存，启动时恢复。
#[derive(Default)]
pub(crate) struct Settings {
    pub(crate) playlist: Vec<PathBuf>,
    pub(crate) position: f32,
    pub(crate) volume: f32,
    pub(crate) mode: PlaybackMode,
    pub(crate) pin: bool,
    pub(crate) current: Option<PathBuf>,
    /// 均衡器参数（当前版本无 UI，仅前向兼容存储；EQ 模块接入后由界面更新）。
    pub(crate) eq: EqSettings,
}

/// 设置文件路径：%APPDATA%\zzhMusicPlayer\settings.txt。
pub(crate) fn settings_path() -> PathBuf {
    let base = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    base.join("zzhMusicPlayer").join("settings.txt")
}

pub(crate) fn load_settings() -> Settings {
    let mut s = Settings {
        volume: 1.0,
        mode: PlaybackMode::Sequential,
        ..Default::default()
    };
    let Ok(text) = std::fs::read_to_string(settings_path()) else {
        return s;
    };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
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
            // eq=启用,增益0,增益1,...,增益9（dB）
            "eq" => {
                let mut it = value
                    .split(',')
                    .map(|v| v.trim().parse::<f32>().unwrap_or(0.0));
                s.eq.enabled = it.next().unwrap_or(0.0) > 0.5;
                for (i, g) in it.enumerate() {
                    if i < EqSettings::default().gains.len() {
                        s.eq.gains[i] = g.clamp(-12.0, 12.0);
                    }
                }
            }
            "playlist" => s.playlist.push(PathBuf::from(value)),
            _ => {}
        }
    }
    s
}

pub(crate) fn save_settings(
    playlist: &[PathBuf],
    position: f32,
    volume: f32,
    mode: PlaybackMode,
    pin: bool,
    current: Option<&PathBuf>,
    eq: &EqSettings,
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
    let gains = eq
        .gains
        .iter()
        .map(|g| format!("{g:.2}"))
        .collect::<Vec<_>>()
        .join(",");
    out.push_str(&format!(
        "volume={volume}\nposition={position}\nmode={mode}\npin={}\neq={},{}\n",
        u8::from(pin),
        u8::from(eq.enabled),
        gains
    ));
    if let Some(cur) = current {
        out.push_str(&format!("current={}\n", cur.display()));
    }
    for p in playlist {
        out.push_str(&format!("playlist={}\n", p.display()));
    }
    let _ = std::fs::write(path, out);
}
