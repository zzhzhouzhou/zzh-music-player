//! 波形域：后台工作线程、波形结果类型、RAM 缓存写入与 UI 应用。

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use slint::{Image, Model, Rgba8Pixel, SharedPixelBuffer, VecModel};

use crate::PlaylistState;
use crate::TransportState;
use crate::render_utils::{format_time, push_background, render_background};
use crate::transport::ThemeTween;
use crate::waveform_cache::{wave_cache_key, write_wave_cache};
use crate::waveform_generator;

/// 波形生成结果（后台线程产出，UI 线程消费；SharedPixelBuffer 为 Send）。
/// 相比旧版的两张全宽位图，这里只保存 160 个条形高度与小尺寸封面缩略图，
/// 单首占用从约 1MB 降到 100KB 以内。
#[derive(Clone)]
pub struct WaveformResult {
    pub path: PathBuf,
    /// UI 波形条的相对高度（0.0 ~ 1.0）。
    pub bars: Vec<f32>,
    pub duration: Duration,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub theme: [u8; 3],
    pub cover: Option<SharedPixelBuffer<Rgba8Pixel>>,
    /// 高度模糊的封面背景位图（无封面时为 None，回退主题色渐变）。
    pub bg: Option<SharedPixelBuffer<Rgba8Pixel>>,
}

/// 波形结果缓存上限：拖入大量文件时只保留最近若干份，避免内存无限增长。
/// 波形结果如今只含条形高度数组与小尺寸封面缩略图（每份 <100KB），
/// 缓存 8 首也远小于旧版位图方案的 4 首。
pub const WAVE_CACHE_LIMIT: usize = 8;

/// 把波形结果写入 RAM 缓存（LRU 淘汰最旧）；已在缓存则仅覆盖内容。
/// 事件泵的两条路径（磁盘缓存命中 / 后台分析完成）共用，避免淘汰逻辑分叉。
pub fn cache_insert(
    cache: &mut HashMap<PathBuf, WaveformResult>,
    order: &mut VecDeque<PathBuf>,
    path: PathBuf,
    res: WaveformResult,
) {
    if !cache.contains_key(&path) {
        order.push_back(path.clone());
        if order.len() > WAVE_CACHE_LIMIT
            && let Some(oldest) = order.pop_front()
        {
            cache.remove(&oldest);
        }
    }
    cache.insert(path, res);
}

/// 启动波形后台线程：接收文件路径，解码生成波形条、封面与主题色。
/// 队列采用“最新任务优先”：收走积压任务只保留最新；新任务到达时还会使
/// 正在进行的旧分析在下一个音频包边界立即取消，把 CPU 让给当前歌曲。
pub fn spawn_waveform_worker() -> (Sender<PathBuf>, Receiver<WaveformResult>) {
    let (job_tx, job_rx) = mpsc::channel::<PathBuf>();
    let (res_tx, res_rx) = mpsc::channel::<WaveformResult>();
    let cancel = waveform_generator::CancelToken::new();
    std::thread::Builder::new()
        .name("waveform".to_string())
        .spawn(move || {
            while let Ok(mut path) = job_rx.recv() {
                // 在真正开始解码前收走队列，只保留最新歌曲。
                while let Ok(newest) = job_rx.try_recv() {
                    path = newest;
                }
                // 有新任务到来时旧分析立即中止（代次过期）；
                // analyze 内部以最新代次创建检查器，不受自身 cancel 影响。
                cancel.cancel_all();
                let result = waveform_generator::analyze(&path, &cancel).map(|wf| {
                    let bars = waveform_generator::bars_from_columns(
                        &wf.columns,
                        waveform_generator::WAVE_BARS,
                    );
                    WaveformResult {
                        path,
                        bars,
                        duration: wf.duration,
                        title: wf.title,
                        artist: wf.artist,
                        theme: wf.theme,
                        cover: wf.cover,
                        bg: wf.bg,
                    }
                });
                match result {
                    Ok(res) => {
                        // 先落盘缓存（后台线程 IO），再交付 UI；下次播放同曲零解码。
                        write_wave_cache(&res);
                        if res_tx.send(res).is_err() {
                            break;
                        }
                    }
                    Err(e) if e == waveform_generator::CANCELLED => {}
                    Err(e) => eprintln!("波形生成失败: {e}"),
                }
            }
        })
        .expect("无法创建波形线程");
    (job_tx, res_rx)
}

/// 预取播放列表中下一首的波形（当前曲目波形就绪后调用）。
/// 磁盘或内存已有缓存则跳过；用户切歌时后台取消机制会自动让路。
pub fn prefetch_next_track(
    playlist_state: &PlaylistState,
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    waveform_cache: &Rc<RefCell<HashMap<PathBuf, WaveformResult>>>,
    wave_tx: &Sender<PathBuf>,
) {
    let len = playlist.borrow().len();
    if len <= 1 {
        return;
    }
    let cur = playlist_state.get_playlist_current();
    let next = ((cur + 1).rem_euclid(len as i32)) as usize;
    let path = playlist.borrow()[next].clone();
    if waveform_cache.borrow().contains_key(&path) {
        return;
    }
    if let Some((cache_path, _)) = wave_cache_key(&path)
        && cache_path.is_file()
    {
        return;
    }
    let _ = wave_tx.send(path);
}

/// 把波形结果应用到 UI：波形条、封面、时长、元数据与主题渐变背景。
pub fn apply_waveform(
    transport: &TransportState,
    res: &WaveformResult,
    bars_model: &Rc<VecModel<f32>>,
    bg_front: &Cell<bool>,
    theme: &ThemeTween,
) {
    // 行数一致时逐行更新（保留 Slint 行元素复用，波形条平滑过渡到新形状）。
    if bars_model.row_count() == res.bars.len() {
        for (i, v) in res.bars.iter().enumerate() {
            bars_model.set_row_data(i, *v);
        }
    } else {
        bars_model.set_vec(res.bars.clone());
    }
    match &res.cover {
        Some(buf) => {
            transport.set_cover_image(Image::from_rgba8(buf.clone()));
            transport.set_has_cover(true);
        }
        None => {
            transport.set_cover_image(Image::default());
            transport.set_has_cover(false);
        }
    }
    transport.set_duration(res.duration.as_secs_f32());
    transport.set_duration_text(format_time(res.duration.as_secs_f32()));
    // 元数据缺失时退回文件名作为标题。
    let title = res
        .title
        .clone()
        .or_else(|| {
            res.path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .unwrap_or_default();
    transport.set_track_title(title.into());
    transport.set_track_artist(res.artist.clone().unwrap_or_default().into());
    // 主题色补间（约 400ms 过渡）+ 交叉淡入背景。
    // 有封面用高模糊封面位图（中央横带覆盖 + 统一压暗），无封面回退主题色渐变。
    theme.start(res.theme);
    let bg = match &res.bg {
        Some(buf) => Image::from_rgba8(buf.clone()),
        None => Image::from_rgba8(render_background(res.theme)),
    };
    push_background(transport, bg, bg_front);
}
