//! 跨线程事件的类型与后台生产者：文件事件（拖拽/双击/滚轮/单例转发）与更新事件。
//! 通道的统一消费点在 pumps 模块的事件泵。

use std::path::PathBuf;
use std::sync::mpsc::Sender;

use crate::version::app_version;

/// 更新安装包在临时目录中的文件名。
pub const UPDATE_INSTALLER_NAME: &str = "zzhMusicPlayer_Setup-update.exe";

/// 文件夹拖入扫描的单批文件数：搜到一批就交给 UI 渐进式追加。
pub const FOLDER_SCAN_BATCH: usize = 50;
/// 文件夹扫描的单次上限（防止误拖整个盘符导致无限扫描）。
pub const FOLDER_SCAN_MAX_FILES: usize = 10000;

/// 文件相关外部事件（OS 拖拽 / 双击 / 滚轮 / 单例转发），经通道由 UI 线程统一处理。
pub enum FileEvent {
    Dropped(Vec<PathBuf>),
    /// 文件夹后台扫描产出的音频文件批次（每 50 个一批，渐进式加入列表）。
    DroppedBatch(Vec<PathBuf>),
    /// 已运行实例通过 WM_COPYDATA 转发的“用本播放器打开”文件。
    OpenFiles(Vec<PathBuf>),
    DoubleClick,
    Wheel(i32),
    /// 关闭请求（右上角按钮或系统 WM_CLOSE）。
    CloseRequest,
}

/// 更新流程事件：检查/下载线程产出，UI 线程事件泵消费。
#[derive(Clone)]
pub enum UpdateEvent {
    Checking,
    UpToDate,
    Available {
        version: String,
        auto_download: bool,
    },
    Progress(f32),
    Ready,
    Failed(String),
}

/// 后台线程检查更新；结果经通道交 UI 线程。
pub fn spawn_update_check(tx: Sender<UpdateEvent>, auto_download: bool) {
    let _ = std::thread::Builder::new()
        .name("update-check".into())
        .spawn(move || {
            let _ = tx.send(UpdateEvent::Checking);
            match crate::updater::check_latest() {
                Ok(v) if crate::updater::version_newer(app_version(), &v) => {
                    let _ = tx.send(UpdateEvent::Available {
                        version: v,
                        auto_download,
                    });
                }
                Ok(_) => {
                    let _ = tx.send(UpdateEvent::UpToDate);
                }
                Err(e) => {
                    let _ = tx.send(UpdateEvent::Failed(e));
                }
            }
        });
}

/// 后台线程下载安装包到临时目录，进度经通道上报。
pub fn spawn_update_download(tx: Sender<UpdateEvent>) {
    let _ = std::thread::Builder::new()
        .name("update-download".into())
        .spawn(move || {
            let dest = std::env::temp_dir().join(UPDATE_INSTALLER_NAME);
            let progress_tx = tx.clone();
            let result = crate::updater::download_installer(&dest, &move |frac| {
                let _ = progress_tx.send(UpdateEvent::Progress(frac));
            });
            let _ = match result {
                Ok(()) => tx.send(UpdateEvent::Ready),
                Err(e) => tx.send(UpdateEvent::Failed(e)),
            };
        });
}

/// 后台递归扫描文件夹中的音频文件，每凑满一批（50 个）就发回 UI 渐进式追加，
/// 大文件夹也能立刻看到列表在增长。递归深度上限 6 层，总量上限 1 万个。
pub fn spawn_folder_scan(root: PathBuf, tx: Sender<FileEvent>) {
    let _ = std::thread::Builder::new()
        .name("folderscan".to_string())
        .spawn(move || {
            const AUDIO_EXTS: [&str; 6] = ["mp3", "flac", "wav", "ogg", "m4a", "aac"];
            let mut stack: Vec<(PathBuf, usize)> = vec![(root, 0)];
            let mut batch: Vec<PathBuf> = Vec::new();
            let mut total = 0usize;
            'outer: while let Some((dir, depth)) = stack.pop() {
                let Ok(read_dir) = std::fs::read_dir(&dir) else {
                    continue;
                };
                let mut entries: Vec<_> = read_dir.filter_map(Result::ok).collect();
                entries.sort_by_key(|e| e.file_name());
                for entry in entries {
                    let p = entry.path();
                    if p.is_dir() {
                        if depth < 6 {
                            stack.push((p, depth + 1));
                        }
                    } else if p.extension().is_some_and(|ext| {
                        AUDIO_EXTS.contains(&ext.to_ascii_lowercase().to_string_lossy().as_ref())
                    }) {
                        batch.push(p);
                        total += 1;
                        if batch.len() >= FOLDER_SCAN_BATCH {
                            let _ = tx.send(FileEvent::DroppedBatch(std::mem::take(&mut batch)));
                        }
                        if total >= FOLDER_SCAN_MAX_FILES {
                            break 'outer;
                        }
                    }
                }
            }
            if !batch.is_empty() {
                let _ = tx.send(FileEvent::DroppedBatch(batch));
            }
        });
}
