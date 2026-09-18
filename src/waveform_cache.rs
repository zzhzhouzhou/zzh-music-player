//! 波形、封面与背景��位图的磁盘缓存。

use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use slint::{Rgba8Pixel, SharedPixelBuffer};

use crate::settings::settings_path;
use crate::waveform::WaveformResult;
use crate::waveform_generator;

/// 波形磁盘缓存目录大小上限：超过后按“最早使用”优先删除（LRU）。
/// 每条缓存约 1~2KB（160 根波形条 + 元数据 + 封面缩略图 PNG），
/// 50MB 足够存放上万首歌曲的缓存。
const WAVE_CACHE_CAP: u64 = 50 * 1024 * 1024;
/// 波形磁盘缓存文件魔数与版本。
/// v4：背景改中央横带覆盖式模糊并统一压暗（深色 UI），旧缓存自动失效。
const WAVE_CACHE_MAGIC: &[u8; 4] = b"ZWFC";
const WAVE_CACHE_VERSION: u8 = 4;
/// FNV-1a 64 位哈希（缓存文件名用：源路径 + 修改时间）。
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// 波形磁盘缓存目录：%APPDATA%\zzhMusicPlayer\wavecache。
pub(crate) fn wave_cache_dir() -> PathBuf {
    settings_path()
        .parent()
        .unwrap_or(Path::new("."))
        .join("wavecache")
}

/// 计算源文件对应的缓存文件路径与当前修改时间（秒）。
/// 文件名只哈希源路径；修改时间存在文件内部（写入与读取时校验），
/// 源文件被替换后同键命中即检测过期并就地删除，不会残留孤儿缓存。
pub(crate) fn wave_cache_key(path: &Path) -> Option<(PathBuf, u64)> {
    let meta = std::fs::metadata(path).ok()?;
    let mtime = meta
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let name = format!(
        "{:016x}.waveform",
        fnv1a64(path.to_string_lossy().as_bytes())
    );
    Some((wave_cache_dir().join(name), mtime))
}

/// 小端写辅助。
struct CacheWriter(Vec<u8>);

impl CacheWriter {
    fn new() -> Self {
        Self(Vec::with_capacity(2048))
    }
    fn bytes(mut self, b: &[u8]) -> Self {
        self.0.extend_from_slice(b);
        self
    }
    fn u8v(mut self, v: u8) -> Self {
        self.0.push(v);
        self
    }
    fn u16v(mut self, v: u16) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn u32v(mut self, v: u32) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn u64v(mut self, v: u64) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    fn f32v(mut self, v: f32) -> Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    /// 可选字符串：u8 存在标志 + u16 长度 + UTF-8 字节。
    fn opt_str(self, s: &Option<String>) -> Self {
        match s {
            Some(t) => {
                let bytes = t.as_bytes();
                let len = bytes.len().min(u16::MAX as usize) as u16;
                self.u8v(1).u16v(len).bytes(&bytes[..len as usize])
            }
            None => self.u8v(0),
        }
    }
}

/// 小端读辅助。
struct CacheReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> CacheReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.pos + n > self.data.len() {
            return None;
        }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Some(s)
    }
    fn u8v(&mut self) -> Option<u8> {
        self.take(1).map(|s| s[0])
    }
    fn u16v(&mut self) -> Option<u16> {
        self.take(2).map(|s| u16::from_le_bytes([s[0], s[1]]))
    }
    fn u32v(&mut self) -> Option<u32> {
        self.take(4)
            .map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn u64v(&mut self) -> Option<u64> {
        let s = self.take(8)?;
        Some(u64::from_le_bytes([
            s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],
        ]))
    }
    fn f32v(&mut self) -> Option<f32> {
        let s = self.take(4)?;
        Some(f32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn opt_str(&mut self) -> Option<Option<String>> {
        match self.u8v()? {
            0 => Some(None),
            _ => {
                let len = self.u16v()? as usize;
                let bytes = self.take(len)?;
                Some(Some(String::from_utf8_lossy(bytes).into_owned()))
            }
        }
    }
}

/// 把 SharedPixelBuffer 位图编码为 PNG（缓存落盘用）。
fn encode_png(buf: &SharedPixelBuffer<Rgba8Pixel>) -> Option<Vec<u8>> {
    let img = image::RgbaImage::from_raw(buf.width(), buf.height(), buf.as_bytes().to_vec())?;
    let mut png = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    Some(png)
}

/// 把波形分析结果写入磁盘缓存（后台线程调用）。
/// 内容：魔数 + 版本 + 源 mtime + 源路径 + 时长 + 波形条 + 主题色 +
/// 标题/艺术家 + 封面缩略图与模糊背景（PNG 压缩，几十 KB 内）。
pub(crate) fn write_wave_cache(res: &WaveformResult) {
    let Some((cache_path, mtime)) = wave_cache_key(&res.path) else {
        return;
    };
    let cover_png = res.cover.as_ref().and_then(encode_png);
    let bg_png = res.bg.as_ref().and_then(encode_png);
    let mut w = CacheWriter::new()
        .bytes(WAVE_CACHE_MAGIC)
        .u8v(WAVE_CACHE_VERSION)
        .u64v(mtime);
    let path_string = res.path.to_string_lossy().into_owned();
    let path_bytes = path_string.as_bytes();
    w = w.u16v(path_bytes.len().min(u16::MAX as usize) as u16);
    w = w.bytes(&path_bytes[..path_bytes.len().min(u16::MAX as usize)]);
    w = w
        .f32v(res.duration.as_secs_f32())
        .u16v(res.bars.len() as u16);
    for &b in &res.bars {
        w = w.f32v(b);
    }
    w = w.bytes(&res.theme);
    w = w.opt_str(&res.title).opt_str(&res.artist);
    for png in [&cover_png, &bg_png] {
        match png {
            Some(data) => w = w.u8v(1).u32v(data.len() as u32).bytes(data),
            None => w = w.u8v(0),
        }
    }
    let _ = std::fs::create_dir_all(cache_path.parent().unwrap_or(Path::new(".")));
    let tmp = cache_path.with_extension("tmp");
    if std::fs::write(&tmp, w.0).is_ok() {
        // 原子替换：写临时文件再改名，避免读到半截缓存。
        let _ = std::fs::rename(&tmp, &cache_path);
    }
}

/// 从缓存流读取一张可选 PNG 位图（封面缩略图 / 模糊背景共用）。
fn read_cache_png(r: &mut CacheReader) -> Option<Option<SharedPixelBuffer<Rgba8Pixel>>> {
    match r.u8v()? {
        0 => Some(None),
        _ => {
            let len = r.u32v()? as usize;
            let png = r.take(len)?;
            let img = image::load_from_memory(png).ok()?;
            let rgba = img.to_rgba8();
            let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(rgba.width(), rgba.height());
            buf.make_mut_bytes().copy_from_slice(rgba.as_raw());
            Some(Some(buf))
        }
    }
}

/// 读取源文件的波形磁盘缓存（UI 线程调用，命中时免去整曲解码）。
/// 键含源文件修改时间：内容被替换过的缓存直接作废删除。
/// 命中时把访问时间刷新到现在（LRU 依据）。
pub(crate) fn read_wave_cache(path: &Path) -> Option<WaveformResult> {
    let (cache_path, mtime) = wave_cache_key(path)?;
    let data = std::fs::read(&cache_path).ok()?;
    let mut r = CacheReader::new(&data);
    if r.take(4)? != WAVE_CACHE_MAGIC || r.u8v()? != WAVE_CACHE_VERSION {
        return None;
    }
    if r.u64v()? != mtime {
        // 源文件已被替换：缓存作废，顺手删除。
        let _ = std::fs::remove_file(&cache_path);
        return None;
    }
    // 源路径（孤儿清理用，此处跳过）。
    let path_len = r.u16v()? as usize;
    let src = String::from_utf8_lossy(r.take(path_len)?).into_owned();
    let duration = Duration::from_secs_f32(r.f32v()?);
    let bars_len = r.u16v()? as usize;
    if bars_len != waveform_generator::WAVE_BARS {
        return None;
    }
    let mut bars = Vec::with_capacity(bars_len);
    for _ in 0..bars_len {
        bars.push(r.f32v()?);
    }
    let theme_bytes = r.take(3)?;
    let theme = [theme_bytes[0], theme_bytes[1], theme_bytes[2]];
    let title = r.opt_str()?;
    let artist = r.opt_str()?;
    let cover = read_cache_png(&mut r)?;
    let bg = read_cache_png(&mut r)?;
    // LRU 触碰：把缓存文件修改时间刷到现在。
    if let Ok(f) = std::fs::OpenOptions::new().write(true).open(&cache_path) {
        let _ = f.set_modified(SystemTime::now());
    }
    let _ = src;
    Some(WaveformResult {
        path: path.to_path_buf(),
        bars,
        duration,
        title,
        artist,
        theme,
        cover,
        bg,
    })
}

/// 启动时的缓存维护（后台线程）：
/// 1. 删除源文件已不存在的孤儿缓存；
/// 2. 总大小超过上限时按修改时间从旧到新删除（保留约 80% 容量）。
pub(crate) fn trim_wave_cache() {
    let dir = wave_cache_dir();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return;
    };
    let mut items: Vec<(SystemTime, u64, PathBuf)> = Vec::new();
    let mut total: u64 = 0;
    for entry in entries.filter_map(Result::ok) {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("waveform") {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        // 读文件头里的源路径，源不存在即为孤儿。
        if let Ok(mut f) = std::fs::File::open(&p) {
            let mut head = vec![0u8; 512];
            let n = f.read(&mut head).unwrap_or(0);
            let mut r = CacheReader::new(&head[..n]);
            let is_orphan = r.take(4).map(|m| m != WAVE_CACHE_MAGIC).unwrap_or(true) || {
                (|| {
                    if r.u8v()? != WAVE_CACHE_VERSION {
                        return Some(true);
                    }
                    let _ = r.u64v()?;
                    let len = r.u16v()? as usize;
                    let src = String::from_utf8_lossy(r.take(len)?).into_owned();
                    Some(!Path::new(&src).is_file())
                })()
                .unwrap_or(true)
            };
            if is_orphan {
                let _ = std::fs::remove_file(&p);
                continue;
            }
        }
        let size = meta.len();
        let modified = meta.modified().unwrap_or(UNIX_EPOCH);
        total += size;
        items.push((modified, size, p));
    }
    if total <= WAVE_CACHE_CAP {
        return;
    }
    items.sort_by_key(|(t, _, _)| *t);
    let target = WAVE_CACHE_CAP * 80 / 100;
    for (_, size, p) in items {
        if total <= target {
            break;
        }
        if std::fs::remove_file(p).is_ok() {
            total = total.saturating_sub(size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 波形磁盘缓存读写回环：写入后应能原样读回，并在源文件变化后失效。
    #[test]
    fn roundtrip_and_source_invalidation() {
        let fake_appdata =
            std::env::temp_dir().join(format!("zzh_cache_test_env_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&fake_appdata);
        std::fs::create_dir_all(&fake_appdata).unwrap();
        unsafe { std::env::set_var("APPDATA", &fake_appdata) };

        let src = fake_appdata.join("song.wav");
        std::fs::write(&src, b"not really audio").unwrap();
        let cover = SharedPixelBuffer::<Rgba8Pixel>::new(4, 4);
        let bg = SharedPixelBuffer::<Rgba8Pixel>::new(8, 8);
        let res = WaveformResult {
            path: src.clone(),
            bars: (0..crate::waveform_generator::WAVE_BARS)
                .map(|i| i as f32 / 1000.0)
                .collect(),
            duration: Duration::from_secs(95),
            title: Some("测试曲目".into()),
            artist: None,
            theme: [1, 2, 3],
            cover: Some(cover),
            bg: Some(bg),
        };
        write_wave_cache(&res);

        let (cache_path, _) = wave_cache_key(&src).expect("cache key");
        assert!(cache_path.is_file(), "缓存文件未生成: {:?}", cache_path);
        let read = read_wave_cache(&src).expect("缓存读取失败");
        assert_eq!(read.bars, res.bars);
        assert_eq!(read.duration, res.duration);
        assert_eq!(read.theme, res.theme);
        assert_eq!(read.title, res.title);
        assert_eq!(read.artist, res.artist);
        assert_eq!(
            read.cover
                .as_ref()
                .map(|image| (image.width(), image.height())),
            Some((4, 4))
        );
        assert_eq!(
            read.bg
                .as_ref()
                .map(|image| (image.width(), image.height())),
            Some((8, 8))
        );

        let file = std::fs::OpenOptions::new().append(true).open(&src).unwrap();
        file.set_modified(SystemTime::now() + Duration::from_secs(5))
            .unwrap();
        drop(file);
        assert!(read_wave_cache(&src).is_none(), "过期缓存应失效");
        assert!(!cache_path.is_file(), "过期缓存应被删除");

        let _ = std::fs::remove_dir_all(&fake_appdata);
    }
}
