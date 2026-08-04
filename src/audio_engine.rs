//! 音频播放引擎：封装 rodio 0.22（DeviceSinkBuilder + Player）的播放、暂停、
//! 跳转、位置查询与播放列表/模式。全部 rodio 访问封闭在音频后台线程内，
//! 通过 mpsc 通道与 UI 线程通信。

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;

use rodio::{Decoder, DeviceSinkBuilder, Player};

/// 位置上报 / 播放结束检测的轮询周期。
const TICK: Duration = Duration::from_millis(100);

/// 播放模式。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PlaybackMode {
    /// 顺序播放，播完列表停止。
    #[default]
    Sequential,
    /// 列表循环。
    ListLoop,
    /// 单曲循环。
    SingleLoop,
    /// 随机播放。
    Random,
}

impl PlaybackMode {
    /// 循环到下一个模式。
    pub fn cycle(self) -> Self {
        match self {
            Self::Sequential => Self::ListLoop,
            Self::ListLoop => Self::SingleLoop,
            Self::SingleLoop => Self::Random,
            Self::Random => Self::Sequential,
        }
    }

    /// 模式的中文名。
    pub fn label(self) -> &'static str {
        match self {
            Self::Sequential => "顺序播放",
            Self::ListLoop => "列表循环",
            Self::SingleLoop => "单曲循环",
            Self::Random => "随机播放",
        }
    }
}

/// UI -> 音频线程的命令。
pub enum Command {
    /// 播放列表中的指定曲目。
    PlayAt(usize),
    /// 整体替换播放列表（保持当前曲目与索引一致）。
    SetPlaylist(Vec<PathBuf>),
    /// 播放/暂停切换。
    Toggle,
    /// 跳转到指定位置。
    Seek(Duration),
    /// 设置音量（0.0 ~ 1.0）。
    SetVolume(f32),
    /// 设置播放模式。
    SetMode(PlaybackMode),
    /// 下一首（按模式语义）。
    Next,
    /// 上一首（按模式语义）。
    Prev,
}

/// 音频线程 -> UI 的事件。
pub enum Event {
    /// 新曲目开始播放。
    TrackStarted { path: PathBuf },
    /// 播放位置更新（约每 100ms 一次）。
    Position(Duration),
    /// 顺序模式播完列表，播放停止。
    Finished,
    /// 错误信息。
    Error(String),
}

/// 音频引擎句柄（UI 线程持有；全部 rodio 访问都在后台线程内）。
pub struct AudioEngine {
    tx: Sender<Command>,
    rx: Receiver<Event>,
    _thread: JoinHandle<()>,
}

impl AudioEngine {
    /// 启动音频后台线程并返回句柄。
    pub fn start() -> Self {
        let (tx, cmd_rx) = mpsc::channel::<Command>();
        let (evt_tx, rx) = mpsc::channel::<Event>();
        let thread = std::thread::Builder::new()
            .name("audio-engine".to_string())
            .spawn(move || engine_loop(cmd_rx, evt_tx))
            .expect("无法创建音频线程");
        Self { tx, rx, _thread: thread }
    }

    /// 发送一条命令（不阻塞）。
    pub fn send(&self, command: Command) {
        let _ = self.tx.send(command);
    }

    /// 取出一条待处理事件；无事件时返回 `None`。
    pub fn try_recv_event(&self) -> Option<Event> {
        self.rx.try_recv().ok()
    }
}

/// 音频线程主循环：处理命令 + 每 100ms 上报位置、检测播放结束。
fn engine_loop(rx: Receiver<Command>, tx: Sender<Event>) {
    let device_sink = match DeviceSinkBuilder::open_default_sink() {
        Ok(sink) => sink,
        Err(e) => {
            let _ = tx.send(Event::Error(format!("无法打开音频输出设备: {e}")));
            return;
        }
    };
    // 设备句柄必须活得比 Player 久（drop 设备即停止播放），故在此作用域内持有。
    let player = Player::connect_new(device_sink.mixer());

    let mut playlist: Vec<PathBuf> = Vec::new();
    let mut index: Option<usize> = None;
    let mut paused = false;
    let mut mode = PlaybackMode::Sequential;
    // 极简 xorshift 随机数状态（随机模式用，避免引入额外依赖）。
    let mut rng = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e3779b97f4a7c15);

    loop {
        match rx.recv_timeout(TICK) {
            Ok(command) => handle_command(
                &player, &mut playlist, &mut index, &mut paused, &mut mode, &mut rng, command, &tx,
            ),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // 周期性 tick：播放中上报位置，播完则按模式切歌或结束。
                if !paused && index.is_some() {
                    if player.empty() {
                        advance_on_finish(
                            &player, &playlist, &mut index, &mut paused, mode, &mut rng, &tx,
                        );
                    } else {
                        let _ = tx.send(Event::Position(player.get_pos()));
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
}

/// 处理一条 UI 命令。
#[allow(clippy::too_many_arguments)]
fn handle_command(
    player: &Player,
    playlist: &mut Vec<PathBuf>,
    index: &mut Option<usize>,
    paused: &mut bool,
    mode: &mut PlaybackMode,
    rng: &mut u64,
    command: Command,
    tx: &Sender<Event>,
) {
    match command {
        Command::PlayAt(i) => {
            if i < playlist.len() {
                *index = Some(i);
                start_and_notify(player, playlist, index, paused, tx);
            }
        }
        Command::SetPlaylist(paths) => {
            // 保留当前曲目在新列表中的位置；若已被移除，则索引置空（当前播放不受影响）。
            let current = index.and_then(|i| playlist.get(i)).cloned();
            *playlist = paths;
            *index = current.as_ref().and_then(|p| playlist.iter().position(|q| q == p));
        }
        Command::Toggle => {
            if *paused {
                player.play();
            } else {
                player.pause();
            }
            *paused = !*paused;
        }
        Command::Seek(pos) => {
            // 播放到结尾后播放源已结束（empty）：若此时拖回进度条，
            // 先重新装载当前/最后一首曲目再跳转，否则会没有声音。
            if player.empty() {
                let target = index.or_else(|| (!playlist.is_empty()).then_some(playlist.len() - 1));
                if let Some(i) = target {
                    if let Err(e) = start_track(player, &playlist[i]) {
                        *index = None;
                        let _ = tx.send(Event::Error(e));
                        return;
                    }
                    *index = Some(i);
                    *paused = false;
                    let _ = tx.send(Event::TrackStarted { path: playlist[i].clone() });
                }
            }
            match player.try_seek(pos) {
                Ok(()) => {
                    // 暂停状态下没有周期 tick，直接上报，保证进度条即时刷新。
                    let _ = tx.send(Event::Position(pos));
                }
                Err(e) => {
                    let _ = tx.send(Event::Error(format!("跳转失败: {e}")));
                }
            }
        }
        Command::SetVolume(volume) => player.set_volume(volume),
        Command::SetMode(m) => *mode = m,
        Command::Next => {
            if let Some(i) = *index {
                if let Some(ni) = next_index(i, playlist.len(), *mode, rng) {
                    *index = Some(ni);
                    start_and_notify(player, playlist, index, paused, tx);
                }
            }
        }
        Command::Prev => {
            if let Some(i) = *index {
                *index = prev_index(i, playlist.len(), *mode, rng);
                start_and_notify(player, playlist, index, paused, tx);
            }
        }
    }
}

/// 按模式计算下一首索引。
fn next_index(i: usize, len: usize, mode: PlaybackMode, rng: &mut u64) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(match mode {
        PlaybackMode::Sequential => {
            // 顺序模式最后一首：Next 不再切歌。
            if i + 1 >= len {
                return None;
            }
            i + 1
        }
        PlaybackMode::ListLoop => (i + 1) % len,
        PlaybackMode::SingleLoop => i,
        PlaybackMode::Random => random_other(rng, i, len),
    })
}

/// 按模式计算上一首索引。
fn prev_index(i: usize, len: usize, mode: PlaybackMode, rng: &mut u64) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(match mode {
        PlaybackMode::Sequential => i.saturating_sub(1),
        PlaybackMode::ListLoop => (i + len - 1) % len,
        PlaybackMode::SingleLoop => i,
        PlaybackMode::Random => random_other(rng, i, len),
    })
}

/// 播放结束后按模式切歌或结束。
fn advance_on_finish(
    player: &Player,
    playlist: &[PathBuf],
    index: &mut Option<usize>,
    paused: &mut bool,
    mode: PlaybackMode,
    rng: &mut u64,
    tx: &Sender<Event>,
) {
    let Some(i) = *index else { return };
    let len = playlist.len();
    match mode {
        PlaybackMode::Sequential => {
            if i + 1 < len {
                *index = Some(i + 1);
                start_and_notify(player, playlist, index, paused, tx);
            } else {
                *index = None;
                let _ = tx.send(Event::Finished);
            }
        }
        PlaybackMode::ListLoop => {
            if len > 0 {
                *index = Some((i + 1) % len);
                start_and_notify(player, playlist, index, paused, tx);
            }
        }
        PlaybackMode::SingleLoop => {
            if len > 0 {
                start_and_notify(player, playlist, index, paused, tx);
            }
        }
        PlaybackMode::Random => {
            if len > 1 {
                *index = Some(random_other(rng, i, len));
                start_and_notify(player, playlist, index, paused, tx);
            } else if len == 1 {
                start_and_notify(player, playlist, index, paused, tx);
            } else {
                *index = None;
                let _ = tx.send(Event::Finished);
            }
        }
    }
}

/// 用列表中 `*index` 指向的曲目替换当前播放源；成功则上报 `TrackStarted`。
fn start_and_notify(
    player: &Player,
    playlist: &[PathBuf],
    index: &mut Option<usize>,
    paused: &mut bool,
    tx: &Sender<Event>,
) {
    let Some(i) = *index else { return };
    // 新曲目总是恢复播放状态，避免“暂停中切歌”后实际在放、状态却显示暂停。
    *paused = false;
    match start_track(player, &playlist[i]) {
        Ok(()) => {
            let _ = tx.send(Event::TrackStarted { path: playlist[i].clone() });
        }
        Err(e) => {
            let _ = tx.send(Event::Error(e));
            *index = None;
        }
    }
}

/// 替换播放源：清空当前源队列并追加新解码器，随后恢复播放。
fn start_track(player: &Player, path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|e| format!("无法打开文件 {:?}: {e}", path))?;
    let decoder = Decoder::try_from(file).map_err(|e| format!("无法解码 {:?}: {e}", path))?;
    player.clear();
    player.append(decoder);
    player.play();
    Ok(())
}

/// 生成 [0, len-1] 内不等于 current 的随机索引（len>1）。
fn random_other(rng: &mut u64, current: usize, len: usize) -> usize {
    let span = (len - 1) as u64;
    let mut idx = (xorshift(rng) % span) as usize;
    if idx >= current {
        idx += 1;
    }
    idx
}

/// xorshift64 伪随机数。
fn xorshift(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}
