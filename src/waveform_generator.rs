//! 波形数据生成：用 symphonia 独立解码音频文件，从真实 PCM 数据
//! 聚合出逐列 min/max 点阵，渲染为波形位图并探测总时长。

use std::fs::File;
use std::path::Path;
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

/// 波形点阵列数（同时也是渲染位图的宽度）。
pub const WAVE_COLUMNS: usize = 2048;

/// 波形位图高度（像素）。
pub const WAVE_HEIGHT: u32 = 64;

/// 波形背景色（半透明白，与深色毛玻璃融合）。
const BG_COLOR: [u8; 3] = [255, 255, 255];
const BG_ALPHA: u8 = 40;

const FG_ALPHA: u8 = 230;

/// 音频波形分析结果：逐列 min/max（保留符号，约 -1.0 ~ 1.0）与元信息。
pub struct Waveform {
    /// 每列一个 `(min, max)` 对，按时间顺序排列。
    pub columns: Vec<(f32, f32)>,
    /// 音频总时长。
    pub duration: Duration,
    /// 从元数据读取的歌曲名（可能缺失）。
    pub title: Option<String>,
    /// 从元数据读取的艺术家（可能缺失）。
    pub artist: Option<String>,
    /// 主题色（来自内嵌封面主色调，或文件名哈希兜底）。
    pub theme: [u8; 3],
    /// 逐列响度（0~255，用于底部律动光带呼吸）。
    pub loudness: Vec<u8>,
}

/// 解码音频文件并生成逐列 min/max 波形点阵，同时读取歌曲元数据。
///
/// 多数容器（MP3/FLAC/WAV/AAC 等）在轨道参数中直接给出总帧数，此时只需解码一遍即可把
/// 样本精确映射到列，速度接近翻倍；仅当帧数未知时才退化为两遍解码。全程流式处理，
/// 不把整首歌曲载入内存。
pub fn analyze(path: &Path) -> Result<Waveform, String> {
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
    let (total_samples, rate, channels) =
        match (n_frames, sample_rate, channel_count) {
            (Some(frames), Some(rate), Some(channels)) if frames > 0 => {
                (frames.saturating_mul(u64::from(channels)), rate, channels)
            }
            _ => {
                let (total, rate, channels) =
                    count_samples(&mut format, &mut decoder, track_id)?;
                format
                    .seek(
                        SeekMode::Accurate,
                        SeekTo::Time {
                            time: Time { seconds: 0, frac: 0.0 },
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

    let columns = aggregate(&mut format, &mut decoder, track_id, total_samples)?;
    let loudness = compute_loudness(&columns);
    let theme = cover
        .as_deref()
        .and_then(theme_from_cover)
        .unwrap_or_else(|| hash_theme(path));

    let seconds = total_samples as f64 / (f64::from(rate) * f64::from(channels));
    Ok(Waveform {
        columns,
        duration: Duration::from_secs_f64(seconds),
        title,
        artist,
        theme,
        loudness,
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

/// 取下一个音频包；读到文件末尾（UnexpectedEof）返回 `Ok(None)`。
fn next_packet(format: &mut Box<dyn FormatReader>) -> Result<Option<Packet>, String> {
    match format.next_packet() {
        Ok(packet) => Ok(Some(packet)),
        Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(format!("读取音频失败: {e}")),
    }
}

/// 第一遍：解码计数 interleaved 样本总数，同时记录采样率与声道数。
fn count_samples(
    format: &mut Box<dyn FormatReader>,
    decoder: &mut Box<dyn Decoder>,
    track_id: u32,
) -> Result<(u64, u32, u16), String> {
    let mut total: u64 = 0;
    let (mut rate, mut channels) = (44100u32, 2u16);
    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    while let Some(packet) = next_packet(format)? {
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                rate = spec.rate;
                channels = spec.channels.count() as u16;
                let buf = sample_buf
                    .get_or_insert_with(|| SampleBuffer::<f32>::new(decoded.capacity() as u64, spec));
                buf.copy_interleaved_ref(decoded);
                total += buf.samples().len() as u64;
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(format!("解码失败: {e}")),
        }
    }
    Ok((total, rate, channels))
}

/// 第二遍：把 interleaved 样本按 `总样本数 / 列数` 分桶，逐列聚合 min/max。
fn aggregate(
    format: &mut Box<dyn FormatReader>,
    decoder: &mut Box<dyn Decoder>,
    track_id: u32,
    total_samples: u64,
) -> Result<Vec<(f32, f32)>, String> {
    let cols = WAVE_COLUMNS;
    let mut mins = vec![f32::MAX; cols];
    let mut maxs = vec![f32::MIN; cols];
    let samples_per_col = (total_samples / cols as u64).max(1) as usize;
    let spc = samples_per_col as u64;

    let mut sample_buf: Option<SampleBuffer<f32>> = None;
    let mut cursor: u64 = 0;
    while let Some(packet) = next_packet(format)? {
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                let buf = sample_buf
                    .get_or_insert_with(|| SampleBuffer::<f32>::new(decoded.capacity() as u64, spec));
                buf.copy_interleaved_ref(decoded);
                for &sample in buf.samples() {
                    let col = ((cursor / spc) as usize).min(cols - 1);
                    mins[col] = mins[col].min(sample);
                    maxs[col] = maxs[col].max(sample);
                    cursor += 1;
                }
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(format!("解码失败: {e}")),
        }
    }

    // 未填充到的列兜底为 0（静音）。
    for (mn, mx) in mins.iter_mut().zip(maxs.iter_mut()) {
        if *mn == f32::MAX {
            *mn = 0.0;
        }
        if *mx == f32::MIN {
            *mx = 0.0;
        }
    }
    Ok(mins.into_iter().zip(maxs).collect())
}

/// 逐列响度：取该列 |min|/|max| 的峰值（0~255），用于底部律动光带呼吸。
fn compute_loudness(columns: &[(f32, f32)]) -> Vec<u8> {
    columns
        .iter()
        .map(|&(mn, mx)| {
            let peak = mn.abs().max(mx.abs()).clamp(0.0, 1.0);
            (peak * 255.0) as u8
        })
        .collect()
}

/// 从内嵌封面提取主色调：解码后缩到 16×16 取平均，再提饱和度/亮度保证渐变好看。
fn theme_from_cover(data: &[u8]) -> Option<[u8; 3]> {
    let img = image::load_from_memory(data).ok()?;
    let small = img.thumbnail(16, 16).to_rgb8();
    let count = small.pixels().count().max(1) as u64;
    let (mut r, mut g, mut b) = (0u64, 0u64, 0u64);
    for p in small.pixels() {
        r += u64::from(p[0]);
        g += u64::from(p[1]);
        b += u64::from(p[2]);
    }
    let rgb = [(r / count) as u8, (g / count) as u8, (b / count) as u8];
    Some(normalize_theme(rgb))
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
fn rgb_to_hsl([r, g, b]: [u8; 3]) -> (f32, f32, f32) {
    let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    let s = if d == 0.0 { 0.0 } else { d / (1.0 - (2.0 * l - 1.0).abs()) };
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
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [u8; 3] {
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

/// 将逐列 min/max 点阵渲染为两张 RGBA 位图：半透明背景波形 + 主题色高亮波形。
/// 返回的像素缓冲可跨线程传递（`Send`），由 UI 线程构造 `slint::Image`。
pub fn render_wave_buffers(
    columns: &[(f32, f32)],
    fg: [u8; 3],
) -> (SharedPixelBuffer<Rgba8Pixel>, SharedPixelBuffer<Rgba8Pixel>) {
    (
        render_buffer(columns, BG_COLOR, BG_ALPHA),
        render_buffer(columns, fg, FG_ALPHA),
    )
}

/// 把点阵绘制成一张竖线波形位图：每列一条竖线，从 min 到 max，
/// 以垂直中心为 0 刻度线对称展开。
fn render_buffer(
    columns: &[(f32, f32)],
    rgb: [u8; 3],
    alpha: u8,
) -> SharedPixelBuffer<Rgba8Pixel> {
    let width = WAVE_COLUMNS as u32;
    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, WAVE_HEIGHT);
    let bytes = buffer.make_mut_bytes();
    let stride = width as usize * 4;
    let center = WAVE_HEIGHT as f32 / 2.0;
    let scale = center - 1.0;
    let max_y = WAVE_HEIGHT as i32 - 1;

    for (x, &(mn, mx)) in columns.iter().take(WAVE_COLUMNS).enumerate() {
        let top = ((center - mx * scale).round() as i32).clamp(0, max_y) as usize;
        let bottom = ((center - mn * scale).round() as i32).clamp(0, max_y) as usize;
        let (lo, hi) = (top.min(bottom), top.max(bottom));
        for y in lo..=hi {
            let i = y * stride + x * 4;
            bytes[i] = rgb[0];
            bytes[i + 1] = rgb[1];
            bytes[i + 2] = rgb[2];
            bytes[i + 3] = alpha;
        }
    }
    buffer
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

        let wf = analyze(&path).unwrap();
        assert_eq!(wf.columns.len(), WAVE_COLUMNS);
        assert_eq!(wf.duration.as_secs(), 2);
        // 元数据从 LIST INFO 中读出。
        assert_eq!(wf.title.as_deref(), Some("Test Title"));
        assert_eq!(wf.artist.as_deref(), Some("Test Artist"));
        // 无封面时主题色来自文件名哈希，响度数组与列数一致。
        assert_eq!(wf.loudness.len(), WAVE_COLUMNS);
        assert!(wf.loudness.iter().any(|&v| v > 0), "正弦波应有响度");

        // 全幅正弦波：正峰值应接近 +1，负谷值接近 -1（证明波形来自真实 PCM）。
        let peak = wf.columns.iter().map(|&(_, mx)| mx).fold(f32::MIN, f32::max);
        let trough = wf.columns.iter().map(|&(mn, _)| mn).fold(f32::MAX, f32::min);
        assert!(peak > 0.9, "峰值过低: {peak}");
        assert!(trough < -0.9, "谷值过浅: {trough}");

        // 渲染位图尺寸与内容验证。
        let (mut bg, fg) = render_wave_buffers(&wf.columns, wf.theme);
        assert_eq!(bg.width(), WAVE_COLUMNS as u32);
        assert_eq!(fg.height(), WAVE_HEIGHT);
        let bytes = bg.make_mut_bytes();
        let stride = WAVE_COLUMNS * 4;
        // 中心刻度线（y = H/2）处应有波形像素（正弦波跨越 0 刻度）。
        assert!(bytes[(WAVE_HEIGHT as usize / 2) * stride + 3] > 0);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn analyze_rejects_garbage() {
        let path = std::env::temp_dir().join("zzh_waveform_garbage.bin");
        std::fs::write(&path, b"this is definitely not audio data at all").unwrap();
        assert!(analyze(&path).is_err());
        let _ = std::fs::remove_file(&path);
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
        for name in ["test.mp3", "test.flac", "test.wav", "test.ogg", "test.m4a", "test.aac"] {
            let path = dir.join(name);
            if !path.exists() {
                continue;
            }
            let wf = analyze(&path).unwrap_or_else(|e| panic!("{name} 解码失败: {e}"));
            assert_eq!(wf.columns.len(), WAVE_COLUMNS, "{name} 列数不符");
            assert!(wf.duration.as_secs() > 0, "{name} 时长为 0");
            assert!(
                wf.columns.iter().any(|&(mn, mx)| mn.abs() > 0.01 || mx.abs() > 0.01),
                "{name} 波形全静音"
            );
            let (mut bg, fg) = render_wave_buffers(&wf.columns, wf.theme);
            assert_eq!(bg.width(), WAVE_COLUMNS as u32);
            assert_eq!(fg.height(), WAVE_HEIGHT);
            assert!(bg.make_mut_bytes()[WAVE_COLUMNS * (WAVE_HEIGHT as usize / 2) * 4 + 3] > 0);
            eprintln!("{name}: 时长 {:.2}s 解码 OK", wf.duration.as_secs_f64());
        }
    }
}
