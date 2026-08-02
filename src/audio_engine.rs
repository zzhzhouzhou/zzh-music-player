//! 音频播放引擎：封装 rodio 0.22（DeviceSinkBuilder + Player）的播放、暂停、
//! 跳转、位置查询与隐式播放队列。全部 rodio 访问封闭在音频后台线程内，
//! 通过 mpsc 通道与 UI 线程通信。

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;

use rodio::{Decoder, DeviceSinkBuilder, Player};

/// 位置上报 / 播放结束检测的轮询周期。
const TICK: Duration = Duration::from_millis(100);

/// UI -> 音频线程的命令。
pub enum Command {
    /// 把文件加入播放队列；若当前空闲则立即开始播放。
    Load(PathBuf),
    /// 播放/暂停切换。
    Toggle,
    /// 跳转到指定位置。
    Seek(Duration),
    /// 设置音量（0.0 ~ 1.0）。
    SetVolume(f32),
}

/// 音频线程 -> UI 的事件。
pub enum Event {
    /// 新曲目开始播放。
    TrackStarted { path: PathBuf },
    /// 播放位置更新（约每 100ms 一次）。
    Position(Duration),
    /// 队列播完（无下一首），播放停止。
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

    let mut queue: Vec<PathBuf> = Vec::new();
    let mut index: Option<usize> = None;
    let mut paused = false;

    loop {
        match rx.recv_timeout(TICK) {
            Ok(command) => {
                handle_command(&player, &mut queue, &mut index, &mut paused, command, &tx)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // 周期性 tick：播放中上报位置，播完则切下一首或结束。
                if !paused && index.is_some() {
                    if player.empty() {
                        advance_or_finish(&player, &queue, &mut index, &mut paused, &tx);
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
fn handle_command(
    player: &Player,
    queue: &mut Vec<PathBuf>,
    index: &mut Option<usize>,
    paused: &mut bool,
    command: Command,
    tx: &Sender<Event>,
) {
    match command {
        Command::Load(path) => {
            queue.push(path);
            if index.is_none() {
                *index = Some(queue.len() - 1);
                start_and_notify(player, queue, index, paused, tx);
            }
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
                let target = index.or_else(|| (!queue.is_empty()).then_some(queue.len() - 1));
                if let Some(i) = target {
                    if let Err(e) = start_track(player, &queue[i]) {
                        *index = None;
                        let _ = tx.send(Event::Error(e));
                        return;
                    }
                    *index = Some(i);
                    *paused = false;
                    let _ = tx.send(Event::TrackStarted { path: queue[i].clone() });
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
    }
}

/// 播放结束后：有下一首则切歌，否则上报 `Finished` 并复位队列。
fn advance_or_finish(
    player: &Player,
    queue: &[PathBuf],
    index: &mut Option<usize>,
    paused: &mut bool,
    tx: &Sender<Event>,
) {
    let Some(i) = *index else {
        return;
    };
    if i + 1 < queue.len() {
        *index = Some(i + 1);
        start_and_notify(player, queue, index, paused, tx);
    } else {
        *index = None;
        let _ = tx.send(Event::Finished);
    }
}

/// 用队列中 `*index` 指向的曲目替换当前播放源；成功则上报 `TrackStarted`。
fn start_and_notify(
    player: &Player,
    queue: &[PathBuf],
    index: &mut Option<usize>,
    paused: &mut bool,
    tx: &Sender<Event>,
) {
    let i = index.unwrap_or(0);
    // 新曲目总是恢复播放状态，避免“暂停中切歌”后实际在放、状态却显示暂停。
    *paused = false;
    match start_track(player, &queue[i]) {
        Ok(()) => {
            let _ = tx.send(Event::TrackStarted { path: queue[i].clone() });
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
