//! 播放列表域：显示模型（过滤子集 + 显示行到真实行的映射）、搜索匹配
//! 与增删播业务操作。UI 模型是真实列表过滤后的子集，排序/删除后必须
//! 全量重建（Slint 的 for 行复用在部分更新下会残留旧行内容，已踩坑）。

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use slint::VecModel;

use crate::audio_engine::{AudioEngine, Command};
use crate::{PlaylistEntry, PlaylistState};

/// 播放列表显示名：文件名，缺失时用完整路径。
pub fn track_name(path: &Path) -> String {
    path.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// 搜索过滤器：优先正则（大小写不敏感），非法正则退回普通子串匹配。
pub enum Matcher {
    Regex(regex_lite::Regex),
    Substring(String),
}

impl Matcher {
    pub fn matches(&self, name: &str) -> bool {
        match self {
            Self::Regex(re) => re.is_match(name),
            Self::Substring(s) => name.to_lowercase().contains(s),
        }
    }
}

pub fn build_matcher(text: &str) -> Option<Matcher> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    match regex_lite::Regex::new(&format!("(?i){t}")) {
        Ok(re) => Some(Matcher::Regex(re)),
        Err(_) => Some(Matcher::Substring(t.to_lowercase())),
    }
}

/// 播放列表的显示视图：完整列表存于 playlist，UI 模型是应用搜索过滤后的
/// 子集。display_map[display] = real 把显示行号映射回真实索引；无过滤时
/// map 保持为空，real_of 按恒等处理，行为与旧版完全一致。
pub struct PlaylistView {
    model: Rc<VecModel<PlaylistEntry>>,
    display_map: Vec<usize>,
    matcher: Option<Matcher>,
}

impl PlaylistView {
    pub fn new(model: Rc<VecModel<PlaylistEntry>>) -> Self {
        Self {
            model,
            display_map: Vec::new(),
            matcher: None,
        }
    }

    /// 是否处于搜索过滤状态（过滤中 UI 禁用拖动排序）。
    pub fn is_filtering(&self) -> bool {
        self.matcher.is_some()
    }

    /// 显示行号 → 完整列表真实索引（无过滤时恒等）。
    pub fn real_of(&self, display: usize) -> usize {
        self.display_map.get(display).copied().unwrap_or(display)
    }

    /// 按当前过滤器重建显示模型（搜索文本变化 / 过滤中增删曲目时调用）。
    pub fn rebuild(&mut self, playlist: &[PathBuf]) {
        self.display_map.clear();
        let mut rows = Vec::with_capacity(playlist.len());
        for (real, p) in playlist.iter().enumerate() {
            let name = track_name(p);
            if self.matcher.as_ref().is_none_or(|m| m.matches(&name)) {
                self.display_map.push(real);
                rows.push(PlaylistEntry {
                    name: name.into(),
                    real: real as i32,
                });
            }
        }
        self.model.set_vec(rows);
    }

    pub fn set_filter(&mut self, text: &str, playlist: &[PathBuf]) {
        self.matcher = build_matcher(text);
        self.rebuild(playlist);
    }

    /// 新增曲目后同步显示模型：过滤中整体重建，否则直接追加。
    pub fn push(&mut self, real: usize, name: &str, playlist: &[PathBuf]) {
        if self.matcher.is_some() {
            self.rebuild(playlist);
        } else {
            self.display_map.push(real);
            self.model.push(PlaylistEntry {
                name: name.into(),
                real: real as i32,
            });
        }
    }

    /// 批量新增后同步（文件夹扫描批次：过滤中只重建一次）。
    pub fn push_many(&mut self, items: &[(usize, String)], playlist: &[PathBuf]) {
        if self.matcher.is_some() {
            self.rebuild(playlist);
            return;
        }
        for (real, name) in items {
            self.display_map.push(*real);
            self.model.push(PlaylistEntry {
                name: name.into(),
                real: *real as i32,
            });
        }
    }

    /// 删除显示行后同步：全量重建（理由同 moved，避免行复用残留）。
    pub fn removed(&mut self, playlist: &[PathBuf]) {
        self.rebuild(playlist);
    }

    /// 重排后同步（仅在无过滤时调用：过滤状态下 UI 已禁用拖动排序）。
    /// 全量重建而非 remove+insert：Slint 的 for 行复用在成对的部分更新下
    /// 可能残留旧行内容，导致显示顺序与真实列表错位（点歌偏移的根源）。
    pub fn moved(&mut self, playlist: &[PathBuf]) {
        self.rebuild(playlist);
    }

    pub fn cleared(&mut self) {
        self.model.set_vec(Vec::new());
        self.display_map.clear();
    }
}

/// 新文件加入播放列表：去重、同步显示模型与引擎、空闲时立即播放。
/// 返回新加入项的索引；已存在则返回 None。
pub fn add_track(
    path: PathBuf,
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    view: &Rc<RefCell<PlaylistView>>,
    playlist_state: &PlaylistState,
    audio: &AudioEngine,
) -> Option<usize> {
    // 过滤非文件（不存在的路径 / 目录），静默跳过。
    if !path.is_file() {
        return None;
    }
    {
        let mut list = playlist.borrow_mut();
        if list.contains(&path) {
            return None;
        }
        list.push(path.clone());
    }
    let idx = playlist.borrow().len() - 1;
    let name = track_name(&path);
    view.borrow_mut().push(idx, &name, &playlist.borrow());
    audio.send(Command::SetPlaylist(playlist.borrow().clone()));
    // 加入播放列表时不预先分析：只在 TrackStarted 后排队当前歌曲，
    // 避免用户连续拖入多首长音频时后台 FIFO 任务阻塞当前歌曲。
    // 当前没有在播曲目时，新加入的文件立即开始播放。
    if playlist_state.get_playlist_current() < 0 {
        playlist_state.set_playlist_current(idx as i32);
        audio.send(Command::PlayAt(idx));
    }
    Some(idx)
}

/// 批量加入播放列表（文件夹扫描批次用）：全部追加后只发一次 SetPlaylist，
/// 空闲时自动播放首个新加入的曲目。
pub fn add_tracks_batch(
    paths: &[PathBuf],
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    view: &Rc<RefCell<PlaylistView>>,
    playlist_state: &PlaylistState,
    audio: &AudioEngine,
) {
    let mut added: Vec<(usize, String)> = Vec::new();
    let mut first: Option<usize> = None;
    {
        let mut list = playlist.borrow_mut();
        for path in paths {
            if !path.is_file() || list.contains(path) {
                continue;
            }
            list.push(path.clone());
            if first.is_none() {
                first = Some(list.len() - 1);
            }
            added.push((list.len() - 1, track_name(path)));
        }
    }
    if !added.is_empty() {
        view.borrow_mut().push_many(&added, &playlist.borrow());
        audio.send(Command::SetPlaylist(playlist.borrow().clone()));
        if playlist_state.get_playlist_current() < 0
            && let Some(i) = first
        {
            playlist_state.set_playlist_current(i as i32);
            audio.send(Command::PlayAt(i));
        }
    }
}

/// 播放列表中的指定曲目。
pub fn play_at(
    index: usize,
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    playlist_state: &PlaylistState,
    audio: &AudioEngine,
) {
    let list = playlist.borrow();
    if index >= list.len() {
        return;
    }
    playlist_state.set_playlist_current(index as i32);
    drop(list);
    audio.send(Command::PlayAt(index));
}

/// “用本播放器打开”/对话框选中的文件：加入列表（已在列表则定位）并立即播放。
/// 空闲时 add_track 已自动开播，无需重复下发。
pub fn play_file_now(
    path: &PathBuf,
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    view: &Rc<RefCell<PlaylistView>>,
    playlist_state: &PlaylistState,
    audio: &AudioEngine,
) {
    let was_idle = playlist_state.get_playlist_current() < 0;
    let added = add_track(path.clone(), playlist, view, playlist_state, audio);
    let idx = added.or_else(|| playlist.borrow().iter().position(|p| p == path));
    // 新增曲目在空闲时由 add_track 自动播放；已存在曲目或正在播放时，
    // 明确调用 play_at，覆盖“停止后重新打开同一文件”的边界情况。
    if let Some(idx) = idx
        && (added.is_none() || !was_idle)
    {
        play_at(idx, playlist, playlist_state, audio);
    }
}

/// 双击窗口（空白区域）：弹出系统原生文件选择对话框（非应用内窗口）。
/// 选中的文件立即播放，而不是只加入列表继续播旧曲。
pub fn open_file_dialog(
    playlist: &Rc<RefCell<Vec<PathBuf>>>,
    view: &Rc<RefCell<PlaylistView>>,
    playlist_state: &PlaylistState,
    audio: &AudioEngine,
) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("音频文件", &["mp3", "flac", "wav", "aac", "m4a", "ogg"])
        .pick_file()
    {
        play_file_now(&path, playlist, view, playlist_state, audio);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slint::Model;

    /// 搜索过滤器：正则优先，非法正则退回子串匹配，大小写不敏感。
    #[test]
    fn matcher_regex_and_fallback() {
        let m = build_matcher("^白.*flac$").unwrap();
        assert!(m.matches("白色封面.flac"));
        assert!(!m.matches("黑色封面.mp3"));

        // 非法正则（未闭合分组）应退回普通子串匹配而不是报错：
        // 字面量 "(白金" 仍能在 "(白金版)" 这类歌名里命中。
        let m = build_matcher("(白金").unwrap();
        assert!(matches!(m, Matcher::Substring(_)));
        assert!(m.matches("歌曲(白金版).mp3"));

        // 大小写不敏感。
        let m = build_matcher("be what").unwrap();
        assert!(m.matches("Be What You Wanna Be - Darin.flac"));

        // 纯空白：不过滤。
        assert!(build_matcher("   ").is_none());
    }

    /// 显示视图：过滤重建与显示行号 → 真实索引的映射。
    #[test]
    fn playlist_view_filter_and_mapping() {
        let model: Rc<VecModel<PlaylistEntry>> = Rc::new(VecModel::default());
        let mut view = PlaylistView::new(model.clone());
        let list = vec![
            PathBuf::from("E:\\m\\白色封面.flac"),
            PathBuf::from("E:\\m\\黑色封面.mp3"),
            PathBuf::from("E:\\m\\无封面.wav"),
        ];
        view.rebuild(&list);
        assert_eq!(view.model.row_count(), 3);
        assert_eq!(view.real_of(2), 2); // 无过滤：恒等映射

        view.set_filter("黑", &list);
        assert_eq!(view.model.row_count(), 1);
        assert_eq!(view.real_of(0), 1); // 显示第 0 行 → 真实第 1 行

        view.set_filter("^白.*flac$", &list);
        assert_eq!(view.model.row_count(), 1);
        assert_eq!(view.real_of(0), 0);

        view.set_filter("", &list);
        assert_eq!(view.model.row_count(), 3);

        // 过滤中删除：以删除后的真实列表重建，映射应保持一致。
        view.set_filter("色", &list);
        assert_eq!(view.model.row_count(), 2);
        let list2: Vec<PathBuf> = list[1..].to_vec(); // 真实列表删掉了第 0 项
        view.removed(&list2);
        assert_eq!(view.model.row_count(), 1);
        assert_eq!(view.model.row_data(0).unwrap().name, "黑色封面.mp3");
        assert_eq!(view.real_of(0), 0);
    }
}
