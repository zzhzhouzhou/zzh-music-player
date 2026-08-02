# zzhMusicPlayer

极简原生桌面音乐播放器 —— 长条形无边框窗口 + Windows 11 亚克力（Acrylic）毛玻璃，
仅保留播放控制与波形进度条。纯 Rust + Slint 实现，无任何 Web 技术栈。

## 功能特性

- **长条形极简窗口**：无边框、有圆角、可拖拽移动，任务栏/桌面空间占用极小
- **Windows 11 原生亚克力毛玻璃**：DWM `DWMSBT_TRANSIENTWINDOW`（Win11 22H2+），
  旧系统自动回退为 Slint 半透明深色背景
- **真实波形**：波形由音频文件的真实 PCM 数据逐列聚合 min/max 生成（2048 列），已播放部分以主题蓝高亮推进，未播部分半透明白
- **进度条拖拽跳转**：拖拽过程中波形高亮跟随手指预览，松开后真正跳转
- **文件拖拽播放**：从资源管理器拖入音频文件立即播放（支持多文件，形成隐式播放队列，播完自动下一首）
- **双击打开**：双击窗口弹出系统原生文件选择对话框


## 模块职责

| 文件 | 职责 |
|---|---|
| `src/main.rs` | 入口与系统集成：窗口创建、亚克力/圆角、置顶、WndProc 子类化 + 文件拖拽、双击 rfd、回调接线、Timer 事件泵 |
| `src/main.slint` | 声明式 UI：长条窗口布局、圆形图标按钮、波形进度区、`export global UIState` |
| `src/audio_engine.rs` | 音频引擎：rodio 播放/暂停/跳转/位置、隐式播放队列、命令/事件通道、线程主循环 |
| `src/waveform_generator.rs` | 波形生成：symphonia 解码 → 逐列 min/max 聚合 → RGBA 位图渲染、时长探测、单元测试 |
| `build.rs` | `slint_build::compile("src/main.slint")` 编译期生成 UI 代码 |

## 构建与运行

**环境要求**：Rust ≥ 1.92（edition 2024）、Windows 11（22H2+ 获得亚克力效果）

```powershell
# 构建（release 含 LTO + strip 优化）
cargo build --release

# 运行
.\target\release\zzhmusicplayer.exe

# 开发调试（保留控制台日志）
cargo run
```

## 使用说明

1. 从资源管理器**拖拽**一个或多个音频文件到窗口 → 立即开始播放（多文件自动排队）
2. **双击**窗口 → 打开系统文件选择器
3. **▶/⏸** 播放/暂停；**⏮/⏭** 快退/快进
4. 在波形上**点击/拖拽** → 跳转播放
5. **📌** 置顶/取消置顶
6. 按住窗口**空白区域拖动** → 移动窗口

## 支持的音频格式

| 格式 | 容器 | 解码器 |
|---|---|---|
| MP3 | MP3 | symphonia-bundle-mp3 |
| FLAC | FLAC | symphonia-bundle-flac |
| WAV | RIFF | symphonia-format-riff + PCM |
| OGG | Ogg | symphonia-format-ogg + Vorbis |
| M4A / AAC | MP4 / ADTS | symphonia-format-isomp4 + AAC |

- **置顶开关**为第五个 UI 元素（需求"仅四元素"与"置顶可选"冲突时按你的选择实现）

