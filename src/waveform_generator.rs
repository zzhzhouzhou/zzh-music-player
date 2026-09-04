//! 波形数据生成：用 symphonia 独立解码音频文件，从真实 PCM 数据
//! 聚合出逐列 min/max 点阵，再降采样为 UI 波形条高度数组；
//! 同时解码内嵌封面为小尺寸缩略图（供 UI 显示）并提取主题色。

use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatOptions, FormatReader, Packet, SeekMode, SeekTo, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey};
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;

use slint::{Rgba8Pixel, SharedPixelBuffer};

/// 波形点阵列数（分析精度；UI 显示前会降采样为 `WAVE_BARS` 根条）。
pub const WAVE_COLUMNS: usize = 2048;

/// UI 波形条数量（窗口 720 宽、绘制区约 680 宽时每条约 4.25px）。
pub const WAVE_BARS: usize = 160;

/// 封面缩略图最长边（像素）。按 48px 显示尺寸的 2~3 倍预留缩放空间，
/// 单张 RGBA 不超过 128×128×4 = 64KB。
const COVER_THUMB_SIZE: u32 = 128;

/// 单列波形数据：min/max 保留符号（约 -1.0 ~ 1.0），rms 为该列均方根（0 ~ 1）。
///
/// 峰值反映瞬态极值，RMS 反映瞬时能量：响度战音乐（重度削波）几乎所有列的
/// 峰值都贴近 1.0 而无信息量，列间动态差异主要由 RMS 承载。
pub struct WaveColumn {
    pub min: f32,
    pub max: f32,
    pub rms: f32,
}

/// 音频波形分析结果：逐列 min/max/rms 点阵与元信息。
pub struct Waveform {
    /// 每列一个 min/max/rms 值，按时间顺序排列。
    pub columns: Vec<WaveColumn>,
    /// 音频总时长。
    pub duration: Duration,
    /// 从元数据读取的歌曲名（可能缺失）。
    pub title: Option<String>,
    /// 从元数据读取的艺术家（可能缺失）。
    pub artist: Option<String>,
    /// 主题色（来自内嵌封面主色调，或文件名哈希兜底）。
    pub theme: [u8; 3],
    /// 内嵌封面缩略图（无封面时为 `None`）。
    pub cover: Option<SharedPixelBuffer<Rgba8Pixel>>,
}

/// 解码取消令牌：保存"当前有效任务"的代次编号。
/// 切歌时递增代次，正在进行的旧任务在下一个音频包边界检测到过期后立即退出，
/// 把 CPU 让给新歌曲的波形分析。
#[derive(Clone)]
pub struct CancelToken(Arc<AtomicU64>);

impl CancelToken {
    pub fn new() -> Self {
        Self(Arc::new(AtomicU64::new(0)))
    }

    /// 生成一个属于当前代次的检查器；调用 `is_cancelled` 对比全局代次。
    /// `analyze` 开始时调用：此后任何 `cancel_all`（新任务到达）
    /// 都会让本次分析在下一个音频包边界停止。
    fn guard(&self) -> CancelGuard {
        CancelGuard {
            token: self.0.clone(),
            generation: self.0.load(Ordering::Relaxed),
        }
    }

    /// 递增代次：使所有已发出的旧任务检查器过期。
    pub fn cancel_all(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for CancelToken {
    fn default() -> Self {
        Self::new()
    }
}

/// 与一次 `analyze` 调用绑定的代次快照。
struct CancelGuard {
    token: Arc<AtomicU64>,
    generation: u64,
}

impl CancelGuard {
    fn is_cancelled(&self) -> bool {
        self.token.load(Ordering::Relaxed) != self.generation
    }
}

/// 解码音频文件并生成逐列 min/max 波形点阵，同时读取歌曲元数据与封面。
///
/// 多数容器（MP3/FLAC/WAV/AAC 等）在轨道参数中直接给出总帧数，此时只需解码一遍即可把
/// 样本精确映射到列，速度接近翻倍；仅当帧数未知时才退化为两遍解码。全程流式处理，
/// 不把整首歌曲载入内存。`cancel` 过期时在音频包边界提前返回 `Err(CANCELLED)`。
pub fn analyze(path: &Path, cancel: &CancelToken) -> Result<Waveform, String> {
    let cancel = cancel.guard();
    let mut format = open_format(path)?;
    let track = format
        .default_track()
        .ok_or_else(|| "文件没有音频轨道".to_string())?;
    let track_id = track.id;
    // 轨道参数在下面的可变借用前先复制出来。
    let n_frames = track.codec_params.n_frames;
    let sample_rate = track.codec_params.sample_rate;
    let channel_count = track.codec_params.channels.map(|c| c.count() as u16);
    let mut decoder = make_decoder(track)?;

    let (title, artist, cover) = read_metadata(&mut format);

    // 优先使用容器给出的帧数，单遍完成；否则两遍（先计数，再回到开头聚合）。
    let (total_samples, rate, channels) = match (n_frames, sample_rate, channel_count) {
        (Some(frames), Some(rate), Some(channels)) if frames > 0 => {
            (frames.saturating_mul(u64::from(channels)), rate, channels)
        }
        _ => {
            let (total, rate, channels) =
                count_samples(&mut format, &mut decoder, track_id, &cancel)?;
            format
                .seek(
                    SeekMode::Accurate,
                    SeekTo::Time {
                        time: Time {
                            seconds: 0,
                            frac: 0.0,
                        },
                        track_id: None,
                    },
                )
                .map_err(|e| format!("无法回到音频开头: {e}"))?;
            (total, rate, channels)
        }
    };
    if total_samples == 0 {
        return Err("音频没有可解码的样本".to_string());
    }

    let columns = aggregate(&mut format, &mut decoder, track_id, total_samples, &cancel)?;
    // symphonia 0.5 的 ID3v2 解析不含图片帧，遇到无法解析的帧还可能丢失其后
    // 的文本帧：MP3 的标题 / 艺术家 / 封面从文件头手动解析兜底补全。
    let (id3_title, id3_artist, id3_cover) = id3_tags(path);
    let cover = cover.or(id3_cover);
    // 封面只保留小尺寸缩略图（显示 + 主题色都用它），原始压缩字节随即丢弃。
    let (cover_thumb, theme) = cover
        .as_deref()
        .and_then(cover_thumbnail)
        .unwrap_or_else(|| (None, hash_theme(path)));

    let seconds = total_samples as f64 / (f64::from(rate) * f64::from(channels));
    Ok(Waveform {
        columns,
        duration: Duration::from_secs_f64(seconds),
        title: title.or(id3_title),
        artist: artist.or(id3_artist),
        theme,
        cover: cover_thumb,
    })
}

/// 读取容器元数据中的歌曲名、艺术家与内嵌封面。
fn read_metadata(
    format: &mut Box<dyn FormatReader>,
) -> (Option<String>, Option<String>, Option<Vec<u8>>) {
    let mut title = None;
    let mut artist = None;
    let mut cover = None;
    let mut metadata = format.metadata();
    let Some(revision) = metadata.skip_to_latest() else {
        return (None, None, None);
    };
    for tag in revision.tags() {
        let value = tag.value.to_string();
        // 部分格式（如 WAV INFO）会带上结尾空字符，统一剥掉再修剪空白。
        let value = value.trim_matches('\0').trim();
        if value.is_empty() {
            continue;
        }
        match tag.std_key {
            Some(StandardTagKey::TrackTitle) if title.is_none() => title = Some(value.into()),
            Some(StandardTagKey::Artist) if artist.is_none() => artist = Some(value.into()),
            _ => {}
        }
    }
    // 封面取第一张图片类内嵌图（常见为封面）。
    if let Some(visual) = revision.visuals().first()
        && visual.media_type.starts_with("image/")
        && !visual.data.is_empty()
    {
        cover = Some(visual.data.to_vec());
    }
    (title, artist, cover)
}

/// 打开文件并用 symphonia 探测格式。
fn open_format(path: &Path) -> Result<Box<dyn FormatReader>, String> {
    let file = File::open(path).map_err(|e| format!("无法打开文件: {e}"))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    symphonia::default::get_probe()
        .format(
            &Hint::new(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map(|probed| probed.format)
        .map_err(|e| format!("无法识别音频格式: {e}"))
}

/// 根据轨道参数创建解码器。
fn make_decoder(track: &Track) -> Result<Box<dyn Decoder>, String> {
    symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| format!("无法创建解码器: {e}"))
}

/// 取消分析时返回的哨兵错误（worker 捕获后静默丢弃，不打印）。
pub const CANCELLED: &str = "\u{0}cancelled";

/// 取下一个音频包；读到文件末尾（UnexpectedEof）返回 `Ok(None)`。
/// 每个包边界检查一次取消令牌，过期即返回 `CANCELLED`。
fn next_packet(
    format: &mut Box<dyn FormatReader>,
    cancel: &CancelGuard,
) -> Result<Option<Packet>, String> {
    if cancel.is_cancelled() {
        return Err(CANCELLED.to_string());
    }
    match format.next_packet() {
        Ok(packet) => Ok(Some(packet)),
        Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
            Ok(None)
        }
        Err(e) => Err(format!("读取音频失败: {e}")),
    }
}

/// 第一遍：解码计数 interleaved 样本总数，同时记录采样率与声道数。
fn count_samples(
    format: &mut Box<dyn FormatReader>,
    decoder: &mut Box<dyn Decoder>,
    track_id: u32,
    cancel: &CancelGuard,
) -> Result<(u64, u32, u16), String> {
    let mut total: u64 = 0;
    let (mut rate, mut channels) = (44100u32, 2u16);
    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    while let Some(packet) = next_packet(format, cancel)? {
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                rate = spec.rate;
                channels = spec.channels.count() as u16;
                let buf = sample_buf.get_or_insert_with(|| {
                    SampleBuffer::<f32>::new(decoded.capacity() as u64, spec)
                });
                buf.copy_interleaved_ref(decoded);
                total += buf.samples().len() as u64;
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(format!("解码失败: {e}")),
        }
    }
    Ok((total, rate, channels))
}

/// 第二遍：把 interleaved 样本按 `总样本数 / 列数` 分桶，
/// 逐列聚合 min/max 与均方根（RMS）。
fn aggregate(
    format: &mut Box<dyn FormatReader>,
    decoder: &mut Box<dyn Decoder>,
    track_id: u32,
    total_samples: u64,
    cancel: &CancelGuard,
) -> Result<Vec<WaveColumn>, String> {
    let cols = WAVE_COLUMNS;
    let mut mins = vec![f32::MAX; cols];
    let mut maxs = vec![f32::MIN; cols];
    // RMS：累计每列平方和与样本数，最后求均方根。
    let mut sqs = vec![0f32; cols];
    let mut cnts = vec![0u32; cols];
    let samples_per_col = (total_samples / cols as u64).max(1) as usize;
    let spc = samples_per_col as u64;

    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    let mut cursor: u64 = 0;
    while let Some(packet) = next_packet(format, cancel)? {
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                let buf = sample_buf.get_or_insert_with(|| {
                    SampleBuffer::<f32>::new(decoded.capacity() as u64, spec)
                });
                buf.copy_interleaved_ref(decoded);
                for &sample in buf.samples() {
                    let col = ((cursor / spc) as usize).min(cols - 1);
                    mins[col] = mins[col].min(sample);
                    maxs[col] = maxs[col].max(sample);
                    sqs[col] += sample * sample;
                    cnts[col] += 1;
                    cursor += 1;
                }
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(format!("解码失败: {e}")),
        }
    }

    // 未填充到的列兜底为 0（静音）。
    Ok((0..cols)
        .map(|i| {
            if cnts[i] == 0 {
                WaveColumn {
                    min: 0.0,
                    max: 0.0,
                    rms: 0.0,
                }
            } else {
                WaveColumn {
                    min: mins[i],
                    max: maxs[i],
                    rms: (sqs[i] / cnts[i] as f32).sqrt(),
                }
            }
        })
        .collect())
}

/// 把逐列 min/max/rms 点阵降采样为 `bars` 根波形条的相对高度（0.0 ~ 1.0）。
///
/// 流程：每根条取 RMS 为主体、峰值少量掺入（保留瞬态冲击感）→ 按全曲
/// 95 分位幅度归一化（只让最响的约 5% 条顶满）→ 动态范围自适应拉伸 →
/// 感知伽马压缩（中低电平更可见）→ 轻度邻域平滑消除刺状跳变。
///
/// 动态范围拉伸针对响度战音乐（重度削波）：那类歌曲几乎所有列的峰值都
/// 贴近 1.0、RMS 也挤在高位窄带（如 0.85 ~ 1.0），归一化后所有条仍然
/// 全高，整条波形成为一个大长方形。此处以全曲 20 分位为地板，按分布
/// 压缩程度自适应地把剩余动态拉伸到整个显示高度——地板越高（分布越
/// 压）拉伸越强；动态正常的歌曲 20 分位很低，几乎不受影响。
pub fn bars_from_columns(columns: &[WaveColumn], bars: usize) -> Vec<f32> {
    let mut peaks = vec![0f32; bars];
    let mut energies = vec![0f32; bars];
    let n = columns.len();
    if n == 0 || bars == 0 {
        return peaks;
    }
    for (i, col) in columns.iter().enumerate() {
        let idx = (i * bars / n).min(bars - 1);
        let peak = col.min.abs().max(col.max.abs());
        if peak > peaks[idx] {
            peaks[idx] = peak;
        }
        if col.rms > energies[idx] {
            energies[idx] = col.rms;
        }
    }
    // RMS 为主体（承载动态起伏），峰值少量掺入（保留瞬态轮廓）。
    let mut amps: Vec<f32> = peaks
        .iter()
        .zip(&energies)
        .map(|(&p, &e)| e * 0.85 + p * 0.15)
        .collect();
    // 95 / 20 分位（降序表：5% 处即“比 95% 条都响”的电平，80% 处即 20 分位）。
    let mut sorted = amps.clone();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let p95 = sorted[((bars as f32 * 0.05) as usize).min(bars - 1)];
    let p20 = sorted[((bars as f32 * 0.80) as usize).min(bars - 1)];
    let norm = if p95 > 1e-4 { 1.0 / p95 } else { 0.0 };
    for v in &mut amps {
        *v = (*v * norm).clamp(0.0, 1.0);
    }
    // 动态范围自适应拉伸（详见函数头注释）。
    let floor = (p20 * norm).clamp(0.0, 1.0);
    let k = ((floor - 0.5) / 0.4).clamp(0.0, 1.0);
    let denom = 1.0 - floor * k;
    if k > 0.0 && denom > 0.05 {
        for v in &mut amps {
            *v = ((*v - floor * k) / denom).clamp(0.0, 1.0);
        }
    }
    // 感知伽马压缩：中低电平更可见，轮廓更饱满。
    for v in &mut amps {
        *v = v.powf(0.75);
    }
    // 轻度平滑：主体权重保留自身形状，两侧各取 1/4 衔接，避免相邻条生硬跳变。
    (0..bars)
        .map(|i| {
            let l = amps[i.saturating_sub(1)];
            let r = amps[(i + 1).min(bars - 1)];
            (amps[i] * 2.0 + l + r) / 4.0
        })
        .collect()
}

/// 从文件起始的 ID3v2 标签手动解析标题 / 艺术家 / 第一张封面（APIC/PIC 帧）。
/// symphonia 0.5 的 ID3v2 支持不含图片帧（遇到无法解析的帧还会中止后续文本帧
/// 的收集），MP3 的元数据与封面由此兜底补全。
fn id3_tags(path: &Path) -> (Option<String>, Option<String>, Option<Vec<u8>>) {
    use std::io::Read;
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return (None, None, None),
    };
    let mut header = [0u8; 10];
    if file.read_exact(&mut header).is_err() || &header[0..3] != b"ID3" {
        return (None, None, None);
    }
    let major = header[3];
    if !(2..=4).contains(&major) {
        return (None, None, None);
    }
    let flags = header[5];
    let size = match syncsafe(&header[6..10]) {
        Some(s) => s as usize,
        None => return (None, None, None),
    };
    if size == 0 || size > 16 * 1024 * 1024 {
        return (None, None, None);
    }
    let mut body = vec![0u8; size];
    if file.read_exact(&mut body).is_err() {
        return (None, None, None);
    }
    parse_id3_body(&body, major, flags)
}

/// 解析 ID3v2 标签体（v2.2/2.3/2.4），返回 (标题, 艺术家, 第一张封面图片)。
fn parse_id3_body(
    body: &[u8],
    major: u8,
    flags: u8,
) -> (Option<String>, Option<String>, Option<Vec<u8>>) {
    let none = (None, None, None);
    // 标签级去同步（v2.3 及以前作用于整个标签体）：FF 00 -> FF。
    let body: Vec<u8> = if flags & 0x80 != 0 {
        deunsync(body)
    } else {
        body.to_vec()
    };
    let body = body.as_slice();
    let mut pos = 0usize;
    // 扩展头（flag 0x40）：v2.4 的大小为同步安全且含自身；v2.3 不含自身 4 字节。
    if flags & 0x40 != 0 && major >= 3 {
        if body.len() < 4 {
            return none;
        }
        let ext = if major == 4 {
            match syncsafe(&body[0..4]) {
                Some(v) => v as usize,
                None => return none,
            }
        } else {
            u32::from_be_bytes([body[0], body[1], body[2], body[3]]) as usize + 4
        };
        pos = ext.min(body.len());
    }
    let mut title = None;
    let mut artist = None;
    let mut cover = None;
    while pos + 10 <= body.len() {
        // v2.2：6 字节帧头（3 字符 ID + 3 字节长度）。
        if major == 2 {
            let id = &body[pos..pos + 3];
            let fsize = (usize::from(body[pos + 3]) << 16)
                | (usize::from(body[pos + 4]) << 8)
                | usize::from(body[pos + 5]);
            if fsize == 0 {
                break;
            }
            let start = pos + 6;
            let end = (start + fsize).min(body.len());
            let data = &body[start..end];
            match &id[..] {
                b"TT2" if title.is_none() => title = id3_text(data),
                b"TP1" if artist.is_none() => artist = id3_text(data),
                b"PIC" if cover.is_none() => cover = parse_v22_pic(data),
                _ => {}
            }
            pos = start + fsize;
            continue;
        }
        // v2.3 / v2.4：10 字节帧头（4 字符 ID + 4 字节长度 + 2 字节标志）。
        let id = &body[pos..pos + 4];
        // 帧头损坏或进入尾部填充区（0x00）：结束扫描。
        if !id[0].is_ascii_uppercase() {
            break;
        }
        let fsize = if major == 4 {
            match syncsafe(&body[pos + 4..pos + 8]) {
                Some(v) => v as usize,
                None => break,
            }
        } else {
            u32::from_be_bytes([body[pos + 4], body[pos + 5], body[pos + 6], body[pos + 7]])
                as usize
        };
        if fsize == 0 {
            break;
        }
        let fflags = u16::from_be_bytes([body[pos + 8], body[pos + 9]]);
        let start = pos + 10;
        pos = start + fsize;
        let mut data = &body[start..pos.min(body.len())];
        match major {
            4 => {
                // 加密 / 压缩帧无法处理：跳过。
                if fflags & 0x000c != 0 {
                    continue;
                }
                // 数据长度指示（flag 0x01）：前 4 字节为实际长度，跳过。
                if fflags & 0x0001 != 0 && data.len() >= 4 {
                    data = &data[4..];
                }
                let clean;
                if fflags & 0x0002 != 0 {
                    // 帧级去同步。
                    clean = deunsync(data);
                    data = &clean;
                }
                match &id[..] {
                    b"TIT2" if title.is_none() => title = id3_text(data),
                    b"TPE1" if artist.is_none() => artist = id3_text(data),
                    b"APIC" if cover.is_none() => cover = parse_apic(data),
                    _ => {}
                }
            }
            _ => {
                // v2.3：压缩 / 加密帧跳过。
                if fflags & 0x00c0 != 0 {
                    continue;
                }
                match &id[..] {
                    b"TIT2" if title.is_none() => title = id3_text(data),
                    b"TPE1" if artist.is_none() => artist = id3_text(data),
                    b"APIC" if cover.is_none() => cover = parse_apic(data),
                    _ => {}
                }
            }
        }
    }
    (title, artist, cover)
}

/// 解码 ID3 文本帧内容（首字节为编码声明，文本可带结尾空字符）。
fn id3_text(data: &[u8]) -> Option<String> {
    let (&enc, rest) = data.split_first()?;
    let text = match enc {
        1 => {
            // UTF-16：按 BOM 判断字节序，逐 2 字节解码。
            let bom_be = rest.starts_with(&[0xfe, 0xff]);
            let units: Vec<u16> = rest
                .chunks_exact(2)
                .map(|c| {
                    if bom_be {
                        u16::from_be_bytes([c[0], c[1]])
                    } else {
                        u16::from_le_bytes([c[0], c[1]])
                    }
                })
                .collect();
            String::from_utf16_lossy(&units)
        }
        2 => {
            // UTF-16BE（无 BOM）。
            let units: Vec<u16> = rest
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16_lossy(&units)
        }
        // 0 = ISO-8859-1，3 = UTF-8：按 UTF-8 宽松解码（对 ASCII 无差别，
        // 中文场景实际多为 UTF-8/UTF-16，纯 latin1 扩展区罕见）。
        _ => String::from_utf8_lossy(rest).into_owned(),
    };
    // 剥掉结尾空字符、首尾空白与 UTF-16 解码残留的 BOM。
    let text = text
        .trim_matches('\0')
        .trim()
        .trim_start_matches('\u{feff}');
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

/// APIC 帧体 -> 图片字节：编码(1) + MIME(latin1\0) + 图片类型(1) + 描述(终止符) + 数据。
fn parse_apic(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 4 {
        return None;
    }
    let enc = data[0];
    let mut p = 1;
    let mime_end = p + data[p..].iter().position(|&b| b == 0)?;
    let mime = String::from_utf8_lossy(&data[p..mime_end]).to_ascii_lowercase();
    if !mime.starts_with("image/") {
        return None;
    }
    p = mime_end + 1; // 跳过 MIME 终止符。
    p += 1; // 图片类型。
    p += terminated_len(data.get(p..)?, enc)?;
    let pic = data.get(p..)?;
    if pic.is_empty() {
        None
    } else {
        Some(pic.to_vec())
    }
}

/// v2.2 PIC 帧体：编码(1) + 格式(3 字符，如 "PNG") + 类型(1) + 描述(终止符) + 数据。
fn parse_v22_pic(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 6 {
        return None;
    }
    let enc = data[0];
    let format = String::from_utf8_lossy(&data[1..4]).to_ascii_uppercase();
    let mut p = 5;
    p += terminated_len(data.get(p..)?, enc)?;
    let pic = data.get(p..)?;
    if pic.is_empty() || (format != "PNG" && format != "JPG") {
        None
    } else {
        Some(pic.to_vec())
    }
}

/// 计算编码字符串（含终止符）的字节长度。
/// 0/3（latin1/UTF-8）单字节 0 终止；1/2（UTF-16）按 2 字节对齐的 00 00 终止。
fn terminated_len(data: &[u8], enc: u8) -> Option<usize> {
    match enc {
        0 | 3 => data.iter().position(|&b| b == 0).map(|i| i + 1),
        1 | 2 => {
            let mut i = 0;
            while i + 1 < data.len() {
                if data[i] == 0 && data[i + 1] == 0 {
                    return Some(i + 2);
                }
                i += 2;
            }
            None
        }
        _ => None,
    }
}

/// 同步安全整数（每字节仅低 7 位有效）-> u32。
fn syncsafe(bytes: &[u8]) -> Option<u32> {
    let b = <[u8; 4]>::try_from(bytes).ok()?;
    Some(
        (u32::from(b[0] & 0x7f) << 21)
            | (u32::from(b[1] & 0x7f) << 14)
            | (u32::from(b[2] & 0x7f) << 7)
            | u32::from(b[3] & 0x7f),
    )
}

/// 去同步：移除 0xFF 之后填充的 0x00。
fn deunsync(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        out.push(data[i]);
        if data[i] == 0xff && data[i + 1..].first() == Some(&0) {
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

/// 解码内嵌封面：缩略图（最长边 ≤128px）+ 从 16×16 均值提取主题色。
/// 解码失败返回 `None`，调用方回退到文件名哈希主题色。
fn cover_thumbnail(data: &[u8]) -> Option<(Option<SharedPixelBuffer<Rgba8Pixel>>, [u8; 3])> {
    let img = image::load_from_memory(data).ok()?;
    // 主题色：缩到 16×16 取平均，再提饱和度/亮度保证渐变好看。
    let small = img.thumbnail(16, 16).to_rgb8();
    let count = small.pixels().count().max(1) as u64;
    let (mut r, mut g, mut b) = (0u64, 0u64, 0u64);
    for p in small.pixels() {
        r += u64::from(p[0]);
        g += u64::from(p[1]);
        b += u64::from(p[2]);
    }
    let theme = normalize_theme([(r / count) as u8, (g / count) as u8, (b / count) as u8]);

    // 缩略图：转为 RGBA 后整体拷入可跨线程传递的像素缓冲。
    let rgba = img.thumbnail(COVER_THUMB_SIZE, COVER_THUMB_SIZE).to_rgba8();
    let (w, h) = rgba.dimensions();
    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(w, h);
    buf.make_mut_bytes().copy_from_slice(rgba.as_raw());
    Some((Some(buf), theme))
}

/// 无封面时用文件名哈希生成固定主题色（同一文件每次启动颜色一致）。
fn hash_theme(path: &Path) -> [u8; 3] {
    let bytes = path.to_string_lossy();
    let mut hash = 0x811c9dc5u32;
    for &b in bytes.as_bytes() {
        hash ^= u32::from(b);
        hash = hash.wrapping_mul(0x01000193);
    }
    let hue = (hash % 360) as f32;
    let sat = 0.58 + ((hash >> 8) % 30) as f32 / 100.0; // 0.58 ~ 0.87
    let light = 0.46 + ((hash >> 16) % 24) as f32 / 100.0; // 0.46 ~ 0.69
    hsl_to_rgb(hue, sat, light)
}

/// 把平均色归一化为饱和、明亮的主题色（HSL 空间调整后转回 RGB）。
fn normalize_theme(rgb: [u8; 3]) -> [u8; 3] {
    let (h, _s, _l) = rgb_to_hsl(rgb);
    hsl_to_rgb(h, 0.72, 0.58)
}

/// RGB -> HSL（h 0~360，s/l 0~1）。
pub fn rgb_to_hsl([r, g, b]: [u8; 3]) -> (f32, f32, f32) {
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    let s = if d == 0.0 {
        0.0
    } else {
        d / (1.0 - (2.0 * l - 1.0).abs())
    };
    let h = if d == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / d).rem_euclid(6.0))
    } else if max == g {
        60.0 * (((b - r) / d) + 2.0)
    } else {
        60.0 * (((r - g) / d) + 4.0)
    };
    (h, s, l)
}

/// HSL -> RGB。
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [u8; 3] {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h.rem_euclid(360.0) / 60.0;
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    [
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;

    /// 合成一段 16-bit 单声道正弦波 WAV（真实可解码音频）。
    /// 含 `fact` 块（使解析走单遍路径）与 `LIST INFO` 元数据（歌曲名/艺术家）。
    fn synth_wav(path: &Path, seconds: u32, rate: u32, freq: f32) {
        let n = seconds * rate;
        let data_len = n * 2;
        let title = b"Test Title\0"; // 11 字节 -> 补齐为 12
        let artist = b"Test Artist\0"; // 12 字节
        let info_len = 4 + (8 + 12) + (8 + 12); // "INFO" + INAM + IART
        let mut bytes = Vec::with_capacity(108 + data_len as usize);
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(100 + data_len).to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
        bytes.extend_from_slice(&1u16.to_le_bytes()); // 单声道
        bytes.extend_from_slice(&rate.to_le_bytes());
        bytes.extend_from_slice(&(rate * 2).to_le_bytes()); // 字节率
        bytes.extend_from_slice(&2u16.to_le_bytes()); // 块对齐
        bytes.extend_from_slice(&16u16.to_le_bytes()); // 位深
        // fact：总帧数（每声道），让 codec_params.n_frames 有值，走单遍解析。
        bytes.extend_from_slice(b"fact");
        bytes.extend_from_slice(&4u32.to_le_bytes());
        bytes.extend_from_slice(&(n as u32).to_le_bytes());
        // LIST INFO 元数据。
        bytes.extend_from_slice(b"LIST");
        bytes.extend_from_slice(&(info_len as u32).to_le_bytes());
        bytes.extend_from_slice(b"INFO");
        bytes.extend_from_slice(b"INAM");
        bytes.extend_from_slice(&(title.len() as u32).to_le_bytes());
        bytes.extend_from_slice(title);
        bytes.push(0); // 奇数长度补齐
        bytes.extend_from_slice(b"IART");
        bytes.extend_from_slice(&(artist.len() as u32).to_le_bytes());
        bytes.extend_from_slice(artist);
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_len.to_le_bytes());
        for i in 0..n {
            let v = (i as f32 / rate as f32 * std::f32::consts::TAU * freq).sin();
            let s = (v * i16::MAX as f32) as i16;
            bytes.extend_from_slice(&s.to_le_bytes());
        }
        let mut file = File::create(path).unwrap();
        file.write_all(&bytes).unwrap();
    }

    #[test]
    fn analyze_synth_wav_roundtrip() {
        let path = std::env::temp_dir().join("zzh_waveform_test.wav");
        synth_wav(&path, 2, 44100, 440.0);

        let wf = analyze(&path, &CancelToken::new()).unwrap();
        assert_eq!(wf.columns.len(), WAVE_COLUMNS);
        assert_eq!(wf.duration.as_secs(), 2);
        // 元数据从 LIST INFO 中读出。
        assert_eq!(wf.title.as_deref(), Some("Test Title"));
        assert_eq!(wf.artist.as_deref(), Some("Test Artist"));
        // 无封面：cover 为 None，主题色来自文件名哈希。
        assert!(wf.cover.is_none());

        // 全幅正弦波：正峰值应接近 +1，负谷值接近 -1（证明波形来自真实 PCM）。
        let peak = wf.columns.iter().map(|c| c.max).fold(f32::MIN, f32::max);
        let trough = wf.columns.iter().map(|c| c.min).fold(f32::MAX, f32::min);
        assert!(peak > 0.9, "峰值过低: {peak}");
        assert!(trough < -0.9, "谷值过浅: {trough}");

        // 波形条：数量、取值范围与归一化（最响的条应接近满高）。
        let bars = bars_from_columns(&wf.columns, WAVE_BARS);
        assert_eq!(bars.len(), WAVE_BARS);
        assert!(bars.iter().all(|&v| (0.0..=1.0).contains(&v)), "波形条越界");
        let max_bar = bars.iter().cloned().fold(0f32, f32::max);
        assert!(max_bar > 0.6, "归一化后最高条过矮: {max_bar}");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn analyze_rejects_garbage() {
        let path = std::env::temp_dir().join("zzh_waveform_garbage.bin");
        std::fs::write(&path, b"this is definitely not audio data at all").unwrap();
        assert!(analyze(&path, &CancelToken::new()).is_err());
        let _ = std::fs::remove_file(&path);
    }

    /// 响度战场景：重度削波音乐几乎所有列的峰值都贴近 1.0（无信息量），
    /// 仅 RMS 保留段落间能量差异。旧策略（纯峰值 + 最大值归一化）会把
    /// 所有条顶满成长条；新策略（RMS 为主体、95 分位归一化）应产生
    /// 段落间明显起伏的波形条。
    #[test]
    fn bars_clipped_music_keeps_dynamics() {
        let n = WAVE_COLUMNS;
        // 前 40% 列为安静段落（RMS 0.4），后 60% 列为响亮段落（RMS 0.9），
        // 两段峰值同为 0.98（模拟削波：峰值通道完全看不出动态）。
        let columns: Vec<WaveColumn> = (0..n)
            .map(|i| {
                let quiet = i < n * 2 / 5;
                WaveColumn {
                    min: -0.98,
                    max: 0.98,
                    rms: if quiet { 0.4 } else { 0.9 },
                }
            })
            .collect();
        let bars = bars_from_columns(&columns, WAVE_BARS);
        let (quiet, loud) = bars.split_at(WAVE_BARS * 2 / 5);
        let quiet_mean = quiet.iter().sum::<f32>() / quiet.len() as f32;
        let loud_mean = loud.iter().sum::<f32>() / loud.len() as f32;
        // 响亮段落应接近满高，安静段落明显更矮，两段差异可辨。
        assert!(loud_mean > 0.8, "响亮段落过矮: {loud_mean}");
        assert!(
            quiet_mean < loud_mean - 0.15,
            "段落间起伏不足: {quiet_mean} vs {loud_mean}"
        );
        assert!(bars.iter().all(|&v| (0.0..=1.0).contains(&v)), "波形条越界");
    }

    /// 响度战音乐（全曲 RMS 挤在 0.82~1.0 窄带、峰值全部削波）：
    /// 未拉伸时所有条都在 0.85 以上、看起来是一个大长方形；
    /// 动态范围拉伸后四段电平应拉开明显差距。
    #[test]
    fn bars_loudness_war_stretches_dynamics() {
        let n = WAVE_COLUMNS;
        let seg_rms = |i: usize| 0.82 + 0.06 * ((i / (n / 4)) % 4) as f32;
        let columns: Vec<WaveColumn> = (0..n)
            .map(|i| WaveColumn {
                min: -0.99,
                max: 0.99,
                rms: seg_rms(i),
            })
            .collect();
        let bars = bars_from_columns(&columns, WAVE_BARS);
        let seg = |s: usize| -> f32 {
            let slice: Vec<f32> = bars
                .iter()
                .skip(s * WAVE_BARS / 4)
                .take(WAVE_BARS / 4)
                .copied()
                .collect();
            slice.iter().sum::<f32>() / slice.len() as f32
        };
        let (q, l) = (seg(0), seg(3));
        assert!(l > 0.8, "最响段应接近满高: {l}");
        assert!(
            q < l - 0.25,
            "响度战拉伸失效，段落仍挤在一起: {q} vs {l}"
        );
        assert!(bars.iter().all(|&v| (0.0..=1.0).contains(&v)), "波形条越界");
    }

    /// 假图片字节（解析器不校验内容，只需可比较）。
    const FAKE_PNG: &[u8] = &[
        0x89, b'P', b'N', b'G', 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    ];

    /// 构造 v2.3/v2.4 APIC 帧体。
    fn apic_body(enc: u8, description: &[u8]) -> Vec<u8> {
        let mut d = Vec::new();
        d.push(enc);
        d.extend_from_slice(b"image/png\0");
        d.push(3); // 封面类型。
        d.extend_from_slice(description);
        d.extend_from_slice(FAKE_PNG);
        d
    }

    /// 组装完整 ID3 标签（header + 帧）。
    fn id3_tag(major: u8, frames: &[u8]) -> Vec<u8> {
        let mut tag = Vec::new();
        tag.extend_from_slice(b"ID3");
        tag.push(major);
        tag.push(0); // 修订号。
        tag.push(0); // 标签级标志（无去同步/扩展头）。
        tag.extend_from_slice(&to_syncsafe(frames.len() as u32));
        tag.extend_from_slice(frames);
        tag
    }

    fn to_syncsafe(v: u32) -> [u8; 4] {
        [
            ((v >> 21) & 0x7f) as u8,
            ((v >> 14) & 0x7f) as u8,
            ((v >> 7) & 0x7f) as u8,
            (v & 0x7f) as u8,
        ]
    }

    /// 组装 v2.3 帧（长度为普通 u32 大端）。
    fn v23_frame(id: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut f = Vec::new();
        f.extend_from_slice(id);
        f.extend_from_slice(&(data.len() as u32).to_be_bytes());
        f.extend_from_slice(&[0, 0]); // 帧标志。
        f.extend_from_slice(data);
        f
    }

    /// 组装 v2.4 帧（长度为同步安全整数）。
    fn v24_frame(id: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut f = Vec::new();
        f.extend_from_slice(id);
        f.extend_from_slice(&to_syncsafe(data.len() as u32));
        f.extend_from_slice(&[0, 0]);
        f.extend_from_slice(data);
        f
    }

    #[test]
    fn id3v23_apic_extracted() {
        // v2.3 + UTF-8 描述 + 前置一个无关文本帧。
        let mut frames = v23_frame(b"TIT2", b"\x03Title Only\x00");
        frames.extend_from_slice(&v23_frame(b"APIC", &apic_body(3, b"cover\x00")));
        let tag = id3_tag(3, &frames);
        let (title, _, cover) = parse_id3_body(&tag[10..], 3, 0);
        assert_eq!(cover.as_deref(), Some(FAKE_PNG));
        assert_eq!(title.as_deref(), Some("Title Only"));
    }

    #[test]
    fn id3v24_apic_extracted() {
        // v2.4：帧长为同步安全整数。
        let frames = v24_frame(b"APIC", &apic_body(0, b"desc\x00"));
        let tag = id3_tag(4, &frames);
        let (_, _, cover) = parse_id3_body(&tag[10..], 4, 0);
        assert_eq!(cover.as_deref(), Some(FAKE_PNG));
    }

    #[test]
    fn id3v22_pic_extracted() {
        // v2.2：6 字节帧头 + 3 字符 "PIC" + 3 字符格式。
        let mut frame = Vec::new();
        frame.extend_from_slice(b"PIC");
        let mut data = Vec::new();
        data.push(0); // latin1 编码。
        data.extend_from_slice(b"PNG");
        data.push(3);
        data.extend_from_slice(b"d\x00");
        data.extend_from_slice(FAKE_PNG);
        frame.extend_from_slice(&((data.len() as u32) << 8).to_be_bytes()[1..4]);
        frame.extend_from_slice(&data);
        let tag = id3_tag(2, &frame);
        let (_, _, cover) = parse_id3_body(&tag[10..], 2, 0);
        assert_eq!(cover.as_deref(), Some(FAKE_PNG));
    }

    #[test]
    fn id3_utf16_description_skipped() {
        // UTF-16 描述（BOM + "a" + 00 00 终止）：图片起点应正确跳过描述。
        let desc = [0xff, 0xfe, b'a', 0x00, 0x00, 0x00];
        let frames = v23_frame(b"APIC", &apic_body(1, &desc));
        let tag = id3_tag(3, &frames);
        let (_, _, cover) = parse_id3_body(&tag[10..], 3, 0);
        assert_eq!(cover.as_deref(), Some(FAKE_PNG));
    }

    #[test]
    fn id3_no_picture_returns_none() {
        let frames = v23_frame(b"TIT2", b"\x03Only Title\x00");
        let tag = id3_tag(3, &frames);
        let (title, artist, cover) = parse_id3_body(&tag[10..], 3, 0);
        assert!(cover.is_none());
        assert!(artist.is_none());
        // 标题应被解析出来（symphonia 解析失效时的兜底来源）。
        assert_eq!(title.as_deref(), Some("Only Title"));
    }

    #[test]
    fn id3_utf16_title_decoded() {
        // UTF-16 BOM 编码的中文标题（"星尘"）与 latin1 艺术家。
        let mut text = vec![1u8]; // UTF-16 编码声明。
        text.extend_from_slice(&[0xff, 0xfe]); // LE BOM。
        for u in "星尘".encode_utf16() {
            text.extend_from_slice(&u.to_le_bytes());
        }
        text.extend_from_slice(&[0, 0]);
        let mut frames = v23_frame(b"TIT2", &text);
        frames.extend_from_slice(&v23_frame(b"TPE1", b"\x03Artist\x00"));
        let tag = id3_tag(3, &frames);
        let (title, artist, _) = parse_id3_body(&tag[10..], 3, 0);
        assert_eq!(title.as_deref(), Some("星尘"));
        assert_eq!(artist.as_deref(), Some("Artist"));
    }

    /// 多格式冒烟验证：设置环境变量 `ZZH_TEST_AUDIO_DIR`（目录内含
    /// test.mp3 / test.flac / test.wav / test.ogg / test.m4a / test.aac）时运行，
    /// 否则自动跳过。用于验证 symphonia 对主流格式的真实解码。
    #[test]
    fn external_formats_smoke() {
        let Ok(dir) = std::env::var("ZZH_TEST_AUDIO_DIR") else {
            return;
        };
        let dir = PathBuf::from(dir);
        for name in [
            "test.mp3",
            "test.flac",
            "test.wav",
            "test.ogg",
            "test.m4a",
            "test.aac",
        ] {
            let path = dir.join(name);
            if !path.exists() {
                continue;
            }
            let wf = analyze(&path, &CancelToken::new())
                .unwrap_or_else(|e| panic!("{name} 解码失败: {e}"));
            assert_eq!(wf.columns.len(), WAVE_COLUMNS, "{name} 列数不符");
            assert!(wf.duration.as_secs() > 0, "{name} 时长为 0");
            assert!(
                wf.columns
                    .iter()
                    .any(|c| c.min.abs() > 0.01 || c.max.abs() > 0.01),
                "{name} 波形全静音"
            );
            let bars = bars_from_columns(&wf.columns, WAVE_BARS);
            assert_eq!(bars.len(), WAVE_BARS, "{name} 波形条数量不符");
            assert!(bars.iter().any(|&v| v > 0.1), "{name} 波形条全静音");
            eprintln!("{name}: 时长 {:.2}s 解码 OK", wf.duration.as_secs_f64());
        }
    }
}
