//! 音频播放引擎：封装 rodio 0.22（DeviceSinkBuilder + Player）的播放、暂停、
//! 跳转、位置查询与播放列表/模式。全部 rodio 访问封闭在音频后台线程内，
//! 通过 mpsc 通道与 UI 线程通信。

use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;

use rodio::{ChannelCount, Decoder, DeviceSinkBuilder, Player, SampleRate, Source};

/// 位置上报 / 播放结束检测的轮询周期（50ms：进度条更流畅）。
const TICK: Duration = Duration::from_millis(50);

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
    /// 从播放列表中移除指定曲目；若移除的正是当前播放曲目，自动切到下一首。
    RemoveAt(usize),
    /// 整体替换播放列表（保持当前曲目与索引一致）。
    SetPlaylist(Vec<PathBuf>),
    /// 更新均衡器参数（运行中的播放源即时生效，无毛刺）。
    SetEq(EqSettings),
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
    /// 已打开曲目的可用时长（在波形分析完成前立即提供）。
    Duration { duration: Duration },
    /// 跳转命令已经在播放源上生效（即使当前处于暂停状态也会发送）。
    SeekApplied { position: Duration },
    /// 播放位置更新（约每 50ms 一次）。
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

// —— 采样管线（阶段 1 地基）：模块在解码器与输出之间插入 DSP 阶段 ——

/// 均衡器频段数（ISO 标准倍频程中心频点）。
pub const EQ_BANDS: usize = 10;
/// 均衡器频段中心频率（Hz）。
pub const EQ_FREQS: [f32; EQ_BANDS] =
    [31.25, 62.5, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0];

/// 均衡器参数（模块可经命令通道即时更新，运行中的播放源无毛刺生效）。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct EqSettings {
    pub enabled: bool,
    /// 各频段增益（dB，-12 ~ +12）。
    pub gains: [f32; EQ_BANDS],
}

/// DSP 阶段：模块在解码器与输出之间插入的采样级处理器。
///
/// 逐采样调用（`sample` 为 -1..1 的 f32，`channel` 为当前声道序号），
/// 返回处理后的采样。需要块处理的算法（如 FFT 频谱）请在内部自行缓冲，
/// 并在 `channel == 0` 且缓冲满时输出。实现必须 `Send + 'static`
/// （跟随播放源进入音频线程）。
pub trait DspStage: Send + 'static {
    /// 处理单个采样。
    fn process(&mut self, sample: f32, channel: usize, channels: usize) -> f32;
    /// 采样率变化（每首曲目开始时至少通知一次；重建依赖采样率的系数）。
    fn sample_rate_changed(&mut self, _rate: u32) {}
}

/// 采样管线：解码器 → DSP 阶段链 → 输出。
///
/// 无阶段时逐采样直通（零分配、零转换）；有阶段时 i16 ↔ f32 转换后
/// 依次过链。声道映射按累计采样序号取模，跨 span 边界不错位。
pub struct PipelineSource<S> {
    inner: S,
    stages: Vec<Box<dyn DspStage>>,
    channels: usize,
    consumed: u64,
    last_rate: u32,
}

impl<S> PipelineSource<S>
where
    S: Source<Item = f32>,
{
    pub fn new(inner: S, stages: Vec<Box<dyn DspStage>>) -> Self {
        let channels = usize::from(inner.channels().get());
        Self {
            inner,
            stages,
            channels,
            consumed: 0,
            last_rate: 0,
        }
    }
}

impl<S> Iterator for PipelineSource<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let raw = self.inner.next()?;
        // 采样率变化通知（每首曲目开始时至少一次；阶段可重建系数）。
        let rate = self.inner.sample_rate().get();
        if rate != self.last_rate {
            self.last_rate = rate;
            for stage in &mut self.stages {
                stage.sample_rate_changed(rate);
            }
        }
        if self.stages.is_empty() {
            self.consumed += 1;
            return Some(raw); // 直通：零转换零分配
        }
        let channel = (self.consumed as usize) % self.channels.max(1);
        let mut sample = raw;
        for stage in &mut self.stages {
            sample = stage.process(sample, channel, self.channels);
        }
        self.consumed += 1;
        Some(sample.clamp(-1.0, 1.0))
    }
}

impl<S> Source for PipelineSource<S>
where
    S: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }
    fn channels(&self) -> ChannelCount {
        self.inner.channels()
    }
    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate()
    }
    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
}

/// RBJ peaking biquad（二阶峰值滤波器，Direct Form I）。
#[derive(Clone, Copy)]
struct Biquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Biquad {
    fn peaking(f0: f32, rate: u32, gain_db: f32, q: f32) -> Self {
        let a = 10f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * f0 / rate as f32;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);
        let a0 = 1.0 + alpha / a;
        Self {
            b0: (1.0 + alpha * a) / a0,
            b1: (-2.0 * cos_w0) / a0,
            b2: (1.0 - alpha * a) / a0,
            a1: (-2.0 * cos_w0) / a0,
            a2: (1.0 - alpha / a) / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
    fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}

/// 均衡器阶段：10 段 peaking 滤波器链，每声道独立状态。
/// 参数经 `Arc<Mutex<EqSettings>>` 与命令通道共享；每 256 采样同步一次
/// （应用延迟 ≤ ~6ms），参数未变或禁用/全零增益时逐采样直通。
#[derive(Clone)]
pub struct EqStage {
    shared: Arc<Mutex<EqSettings>>,
    applied: EqSettings,
    rate: u32,
    channels: usize,
    counter: u32,
    dirty: bool,
    states: Vec<[Biquad; EQ_BANDS]>,
}

impl EqStage {
    pub fn new(shared: Arc<Mutex<EqSettings>>) -> Self {
        Self {
            shared,
            applied: EqSettings::default(),
            rate: 0,
            channels: 2,
            counter: 0,
            dirty: false,
            states: Vec::new(),
        }
    }

    fn rebuild(&mut self) {
        self.states = (0..self.channels.max(1))
            .map(|_| {
                std::array::from_fn(|band| {
                    Biquad::peaking(EQ_FREQS[band], self.rate, self.applied.gains[band], 1.1)
                })
            })
            .collect();
    }
}

impl DspStage for EqStage {
    fn process(&mut self, sample: f32, channel: usize, channels: usize) -> f32 {
        // 周期性同步共享参数（避免逐采样加锁；应用延迟 ≤ ~6ms）。
        self.counter = self.counter.wrapping_add(1);
        if self.counter % 256 == 0 {
            // 先克隆出 owned 参数并释放锁，再改内部状态。
            let fresh = self.shared.lock().ok().map(|p| p.clone());
            if let Some(params) = fresh
                && params != self.applied
            {
                self.applied = params;
                self.dirty = true;
            }
        }
        if !self.applied.enabled {
            return sample;
        }
        if self.applied.gains.iter().all(|g| g.abs() < 0.01) {
            return sample; // 全零增益：直通
        }
        if self.dirty || self.states.len() != channels.max(1) {
            self.channels = channels;
            self.rebuild();
            self.dirty = false;
        }
        let ch = channel.min(self.states.len().saturating_sub(1));
        let mut y = sample;
        for bq in &mut self.states[ch] {
            y = bq.process(y);
        }
        y
    }
    fn sample_rate_changed(&mut self, rate: u32) {
        self.rate = rate;
        self.rebuild();
    }
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
        Self {
            tx,
            rx,
            _thread: thread,
        }
    }

    /// 发送一条命令（不阻塞）。
    pub fn send(&self, command: Command) {
        let _ = self.tx.send(command);
    }

    /// 取出一条待处理事件；无事件时返回 `None`。
    pub fn try_recv_event(&self) -> Option<Event> {
        self.rx.try_recv().ok()
    }

    /// 更新均衡器参数：运行中的播放源在下一个同步周期（≤ ~6ms）生效。
    pub fn set_eq(&self, eq: EqSettings) {
        self.send(Command::SetEq(eq));
    }
}

/// 音量淡变时长（切歌淡出 / 暂停淡出）。
const FADE_OUT_MS: u64 = 220;
/// 音量淡入时长（切歌淡入 / 恢复播放淡入）。
const FADE_IN_MS: u64 = 320;
/// 淡变的步进间隔（每步重设一次音量，30ms 步长听感平滑且 CPU 可忽略）。
const FADE_STEP_MS: u64 = 30;

/// 在 `from` 与 `to` 之间线性淡变播放音量（阻塞音频线程约 ms 毫秒）。
/// 期间到达的 UI 命令在通道中排队，淡变结束后依次处理。
fn fade_volume(player: &Player, from: f32, to: f32, ms: u64) {
    let steps = (ms / FADE_STEP_MS).max(1);
    let delay = Duration::from_millis(ms / steps);
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        player.set_volume(from + (to - from) * t);
        std::thread::sleep(delay);
    }
    player.set_volume(to);
}

/// 音频线程主循环：处理命令 + 每 50ms 上报位置、检测播放结束。
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

    // 均衡器共享状态：命令通道更新，采样管线经 Arc 读取（阶段 1 挂钩）。
    let eq: Arc<Mutex<EqSettings>> = Arc::new(Mutex::new(EqSettings::default()));

    let mut playlist: Vec<PathBuf> = Vec::new();
    let mut index: Option<usize> = None;
    let mut paused = false;
    let mut mode = PlaybackMode::Sequential;
    // UI 侧目标音量（0.0~1.0）：淡变在它与 0 之间进行。
    let mut volume = 1.0f32;
    // 极简 xorshift 随机数状态（随机模式用，避免引入额外依赖）。
    let mut rng = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e3779b97f4a7c15);

    loop {
        match rx.recv_timeout(TICK) {
            Ok(command) => handle_command(
                &player,
                &mut playlist,
                &mut index,
                &mut paused,
                &mut mode,
                &mut volume,
                &mut rng,
                &eq,
                command,
                &tx,
            ),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // 周期性 tick：播放中上报位置，播完则按模式切歌或结束。
                if !paused && index.is_some() {
                    if player.empty() {
                        advance_on_finish(
                            &player,
                            &playlist,
                            &mut index,
                            &mut paused,
                            mode,
                            volume,
                            &eq,
                            &mut rng,
                            &tx,
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
    volume: &mut f32,
    rng: &mut u64,
    eq: &Arc<Mutex<EqSettings>>,
    command: Command,
    tx: &Sender<Event>,
) {
    match command {
        Command::PlayAt(i) => {
            if i < playlist.len() {
                *index = Some(i);
                start_and_notify(player, playlist, index, paused, eq, *volume, tx);
            }
        }
        Command::RemoveAt(i) => {
            if i >= playlist.len() {
                return;
            }
            let was_current = *index == Some(i);
            let current_path = index.and_then(|ci| playlist.get(ci)).cloned();
            playlist.remove(i);
            // 按路径重新定位当前曲目（被删则 index 置空）。
            *index = current_path
                .as_ref()
                .and_then(|p| playlist.iter().position(|q| q == p));
            if was_current {
                if playlist.is_empty() {
                    // 列表已空：停止播放。
                    *index = None;
                    let _ = tx.send(Event::Finished);
                } else {
                    // 正在播放的曲目被删除：自动切到下一首（末尾则回到开头）。
                    let next = index.map_or(0, |ci| (ci + 1) % playlist.len());
                    *index = Some(next);
                    start_and_notify(player, playlist, index, paused, eq, *volume, tx);
                }
            }
        }
        Command::SetPlaylist(paths) => {
            // 保留当前曲目在新列表中的位置；若已被移除，则索引置空（当前播放不受影响）。
            let current = index.and_then(|i| playlist.get(i)).cloned();
            *playlist = paths;
            *index = current
                .as_ref()
                .and_then(|p| playlist.iter().position(|q| q == p));
        }
        Command::SetEq(settings) => {
            // 仅更新共享参数：运行中的采样管线在下一个同步周期读取生效。
            if let Ok(mut p) = eq.lock() {
                *p = settings;
            }
        }
        Command::Toggle => {
            if *paused {
                // 恢复播放：立即出声，音量从 0 平滑淡入到目标。
                player.play();
                *paused = false;
                fade_volume(player, 0.0, *volume, FADE_IN_MS);
            } else {
                // 暂停：音量缓缓淡出到 0 再挂起，听感平滑。
                *paused = true;
                fade_volume(player, *volume, 0.0, FADE_OUT_MS);
                player.pause();
            }
        }
        Command::Seek(pos) => {
            // 播放到结尾后播放源已结束（empty）：若此时拖回进度条，
            // 先重新装载当前/最后一首曲目再跳转，否则会没有声音。
            if player.empty() {
                let target = index.or_else(|| (!playlist.is_empty()).then_some(playlist.len() - 1));
                if let Some(i) = target {
                    if let Ok(duration) = start_track(player, &playlist[i], eq) {
                        *index = Some(i);
                        *paused = false;
                        let _ = tx.send(Event::TrackStarted {
                            path: playlist[i].clone(),
                        });
                        let _ = tx.send(Event::Duration { duration });
                    } else {
                        *index = None;
                        let _ =
                            tx.send(Event::Error(format!("无法重新打开音频 {:?}", playlist[i])));
                        return;
                    }
                }
            }
            match player.try_seek(pos) {
                Ok(()) => {
                    // 直接发送明确的 seek 完成事件。UI 不再把这次操作误判为普通
                    // Position，从而不会在暂停或异步 seek 期间被旧位置上报拉回。
                    let _ = tx.send(Event::SeekApplied { position: pos });
                }
                Err(e) => {
                    let _ = tx.send(Event::Error(format!("跳转失败: {e}")));
                }
            }
        }
        Command::SetVolume(v) => {
            // 记录目标音量（淡变回到的基准）并立即应用；淡变过程中调整音量
            // 会直接生效，下一次淡变以新值为准。
            *volume = v;
            player.set_volume(v);
        }
        Command::SetMode(m) => *mode = m,
        Command::Next => {
            if let Some(i) = *index
                && let Some(ni) = next_index(i, playlist.len(), *mode, rng)
            {
                *index = Some(ni);
                start_and_notify(player, playlist, index, paused, eq, *volume, tx);
            }
        }
        Command::Prev => {
            if let Some(i) = *index {
                *index = prev_index(i, playlist.len(), *mode, rng);
                start_and_notify(player, playlist, index, paused, eq, *volume, tx);
            }
        }
    }
}

/// 手动“下一首”的索引：任何模式下都切换到相邻曲目（单曲循环只影响播完自动循环）。
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
        PlaybackMode::ListLoop | PlaybackMode::SingleLoop => (i + 1) % len,
        PlaybackMode::Random => random_other(rng, i, len),
    })
}

/// 手动“上一首”的索引：任何模式下都切换到相邻曲目。
fn prev_index(i: usize, len: usize, mode: PlaybackMode, rng: &mut u64) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(match mode {
        PlaybackMode::Sequential => i.saturating_sub(1),
        PlaybackMode::ListLoop | PlaybackMode::SingleLoop => (i + len - 1) % len,
        PlaybackMode::Random => random_other(rng, i, len),
    })
}

/// 播放结束后按模式切歌或结束。
#[allow(clippy::too_many_arguments)]
fn advance_on_finish(
    player: &Player,
    playlist: &[PathBuf],
    index: &mut Option<usize>,
    paused: &mut bool,
    mode: PlaybackMode,
    volume: f32,
    eq: &Arc<Mutex<EqSettings>>,
    rng: &mut u64,
    tx: &Sender<Event>,
) {
    let Some(i) = *index else { return };
    let len = playlist.len();
    match mode {
        PlaybackMode::Sequential => {
            if i + 1 < len {
                *index = Some(i + 1);
                start_and_notify(player, playlist, index, paused, eq, volume, tx);
            } else {
                *index = None;
                let _ = tx.send(Event::Finished);
            }
        }
        PlaybackMode::ListLoop => {
            if len > 0 {
                *index = Some((i + 1) % len);
                start_and_notify(player, playlist, index, paused, eq, volume, tx);
            }
        }
        PlaybackMode::SingleLoop => {
            if len > 0 {
                start_and_notify(player, playlist, index, paused, eq, volume, tx);
            }
        }
        PlaybackMode::Random => {
            if len > 1 {
                *index = Some(random_other(rng, i, len));
                start_and_notify(player, playlist, index, paused, eq, volume, tx);
            } else if len == 1 {
                start_and_notify(player, playlist, index, paused, eq, volume, tx);
            } else {
                *index = None;
                let _ = tx.send(Event::Finished);
            }
        }
    }
}

/// 用列表中 `*index` 指向的曲目替换当前播放源；成功则上报 `TrackStarted` 与时长。
/// 切歌带音量淡变：旧曲正在播时缓缓淡出再装载，新曲从 0 缓缓淡入。
fn start_and_notify(
    player: &Player,
    playlist: &[PathBuf],
    index: &mut Option<usize>,
    paused: &mut bool,
    eq: &Arc<Mutex<EqSettings>>,
    volume: f32,
    tx: &Sender<Event>,
) {
    let Some(i) = *index else { return };
    // 新曲目总是恢复播放状态，避免“暂停中切歌”后实际在放、状态却显示暂停。
    let was_playing = !*paused && !player.empty();
    *paused = false;
    // 旧曲淡出（播放源自然结束时已无声音，跳过淡出）。
    if was_playing {
        fade_volume(player, volume, 0.0, FADE_OUT_MS);
    }
    match start_track(player, &playlist[i], eq) {
        Ok(duration) => {
            let path = playlist[i].clone();
            let _ = tx.send(Event::TrackStarted { path });
            let _ = tx.send(Event::Duration { duration });
            // 新曲淡入：阻塞音频线程约 320ms，期间命令排队，听感优先。
            fade_volume(player, 0.0, volume, FADE_IN_MS);
        }
        Err(e) => {
            // 淡出后启动失败：把音量还原，避免下一曲无声。
            player.set_volume(volume);
            let _ = tx.send(Event::Error(e));
            *index = None;
        }
    }
}

/// 替换播放源：打开文件 → 解码器 → 采样管线（模块 DSP 阶段挂点）→ 输出，
/// 随后恢复播放；返回解码器报告的时长。
/// 每首曲目重建管线：DSP 阶段的内部状态（滤波器记忆等）天然复位。
fn start_track(
    player: &Player,
    path: &Path,
    eq: &Arc<Mutex<EqSettings>>,
) -> Result<Duration, String> {
    let file = File::open(path).map_err(|e| format!("无法打开文件 {:?}: {e}", path))?;
    let decoder = Decoder::try_from(file).map_err(|e| format!("无法解码 {:?}: {e}", path))?;
    let duration = decoder.total_duration().unwrap_or_default();
    let stages: Vec<Box<dyn DspStage>> = vec![Box::new(EqStage::new(eq.clone()))];
    let pipeline = PipelineSource::new(decoder, stages);
    player.clear();
    player.append(pipeline);
    player.play();
    Ok(duration)
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

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试用播放源：固定声道数与采样率的 f32 序列（rodio 0.22 的 Source 固定 f32）。
    struct TestSource {
        samples: std::vec::IntoIter<f32>,
        channels: u16,
        rate: u32,
    }

    impl TestSource {
        fn new(samples: Vec<f32>, channels: u16, rate: u32) -> Self {
            Self {
                samples: samples.into_iter(),
                channels,
                rate,
            }
        }
    }

    impl Iterator for TestSource {
        type Item = f32;
        fn next(&mut self) -> Option<f32> {
            self.samples.next()
        }
    }

    impl Source for TestSource {
        fn current_span_len(&self) -> Option<usize> {
            None
        }
        fn channels(&self) -> ChannelCount {
            ChannelCount::new(self.channels).unwrap()
        }
        fn sample_rate(&self) -> SampleRate {
            SampleRate::new(self.rate).unwrap()
        }
        fn total_duration(&self) -> Option<Duration> {
            None
        }
    }

    /// 增益阶段（测试用）：采样 × 系数。
    struct GainStage(f32);
    impl DspStage for GainStage {
        fn process(&mut self, sample: f32, _channel: usize, _channels: usize) -> f32 {
            sample * self.0
        }
    }

    fn sine_stereo(rate: u32, freq: f32, seconds: f32, amp: f32) -> Vec<f32> {
        // 交错立体声：每个声道的第 k 个采样位于时刻 k/rate（声道采样率 = rate）。
        let n = (rate as f32 * seconds) as usize;
        (0..n)
            .map(|i| {
                let t = (i / 2) as f32 / rate as f32;
                (2.0 * std::f32::consts::PI * freq * t).sin() * amp
            })
            .collect()
    }

    fn rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum: f64 = samples.iter().map(|v| f64::from(*v) * f64::from(*v)).sum();
        (sum / samples.len() as f64).sqrt() as f32
    }

    #[test]
    fn pipeline_passthrough_identity() {
        let input: Vec<f32> = (0..1000)
            .map(|i| ((i % 500) as f32 - 250.0) / 500.0)
            .collect();
        let src = TestSource::new(input.clone(), 2, 48000);
        let pipeline = PipelineSource::new(src, Vec::new());
        let out: Vec<f32> = pipeline.collect();
        assert_eq!(out, input, "无阶段时必须逐采样直通");
    }

    #[test]
    fn pipeline_stage_applies_per_channel() {
        // 立体声：左声道 ×0.5，右声道不动——验证声道映射不错位。
        let input: Vec<f32> = (0..400)
            .map(|i| if i % 2 == 0 { 0.5 } else { -0.5 })
            .collect();
        struct HalfLeft;
        impl DspStage for HalfLeft {
            fn process(&mut self, sample: f32, channel: usize, _channels: usize) -> f32 {
                if channel == 0 {
                    sample * 0.5
                } else {
                    sample
                }
            }
        }
        let src = TestSource::new(input.clone(), 2, 48000);
        let pipeline = PipelineSource::new(src, vec![Box::new(HalfLeft)]);
        let out: Vec<f32> = pipeline.collect();
        for (i, o) in out.iter().enumerate() {
            let expect = if i % 2 == 0 { 0.25 } else { -0.5 };
            assert!((*o - expect).abs() < 1e-6, "声道映射错位 at {i}: {o}");
        }
    }



    #[test]
    fn eq_boost_cut_and_bypass() {
        let rate = 48000u32;
        let input = sine_stereo(rate, 1000.0, 1.0, 0.25);
        let in_rms = rms(&input);

        let run_eq = |enabled: bool, gain: f32| -> f32 {
            let shared = Arc::new(Mutex::new(EqSettings {
                enabled,
                gains: {
                    let mut g = [0f32; EQ_BANDS];
                    g[5] = gain; // 1kHz 频段
                    g
                },
            }));
            let src = TestSource::new(input.clone(), 2, rate);
            let stage = EqStage::new(shared);
            let pipeline = PipelineSource::new(src, vec![Box::new(stage)]);
            let out: Vec<f32> = pipeline.collect();
            // 跳过前 1/4（参数同步周期 + 滤波器暂态）。
            let tail = &out[out.len() * 3 / 4..];
            rms(tail)
        };

        let bypass = run_eq(false, 0.0);
        assert!(
            (bypass - in_rms).abs() < in_rms * 0.05,
            "禁用时应直通: bypass={bypass} in={in_rms}"
        );

        let boosted = run_eq(true, 12.0);
        assert!(
            boosted > in_rms * 1.8,
            "1kHz +12dB 应显著提升能量: {boosted} vs {in_rms}"
        );

        let cut = run_eq(true, -12.0);
        assert!(
            cut < in_rms * 0.45,
            "1kHz -12dB 应显著衰减能量: {cut} vs {in_rms}"
        );
    }

    #[test]
    fn eq_stage_usable_as_trait_object() {
        let stages: Vec<Box<dyn DspStage>> =
            vec![Box::new(EqStage::new(Arc::new(Mutex::new(EqSettings::default()))))];
        let src = TestSource::new(sine_stereo(48000, 1000.0, 0.1, 0.25), 2, 48000);
        let pipeline = PipelineSource::new(src, stages);
        let out: Vec<f32> = pipeline.collect();
        assert_eq!(out.len(), 4800, "trait object 管线应正常产出采样");
    }
}
