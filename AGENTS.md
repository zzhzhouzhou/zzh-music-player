# AGENTS.md — zzhMusicPlayer 开发指南

> 本文件写给在此仓库工作的 AI 助手（与人类贡献者）。内容：项目哲学、架构现状、
> 模块化路线图（阶段 0 → A → C）、硬约束与已踩过的坑。**改动前先读完本文件。**

## 项目是什么

Windows 单机音乐播放器。原生 Rust + Slint（femtovg 渲染 + winit 后端），单二进制
约 13MB，无边框长条窗口（默认 720×178，可调整）+ Win11 亚克力毛玻璃。

**项目哲学（所有决策的优先级排序）：**

1. **轻量第一** — 二进制体积、内存、依赖数都抠。禁止引入 reqwest/tokio/serde 这类
   大依赖；能用 `windows-sys` 加 feature 解决就不加 crate；JSON 这类固定结构允许手写
   扫描器（先例：ID3v2 解析器、更新器字段提取）。
2. **美观第二** — 深色玻璃视觉语言：半透明面板 + 发丝描边 + 主题色（从封面提取）点缀。
3. **功能不变原则** — 任何重构/新功能合并前，21 个单元测试必须全绿，且全部 UI 面
   （主界面 / 播放列表抽屉 / 搜索 / 关于 / 拖拽排序）人工过一遍。
4. 中文注释，注释解释 **为什么** 而不是做了什么。

## 构建与验证

```bash
cargo test                  # 21 个单元测试（波形/ID3/更新器/缓存/搜索过滤）
cargo build --release       # 产物 target\release\zzhmusicplayer.exe
# 安装包（改版本后）：
"C:\Users\admin\AppData\Local\Programs\Inno Setup 6\ISCC.exe" installer.iss
```

本地开发完成、准备提交前，必须在 PowerShell 中运行统一质量检查：

```powershell
./scripts/check.ps1
```

该脚本依次执行 `cargo fmt --check`、`cargo check --locked`、`cargo test --locked` 和
`cargo clippy --locked --all-targets -- -D warnings`。GitHub Actions 使用同一脚本，确保本地
行为标准与 CI 一致。CI 只做质量检查，不上传 EXE 或安装包，不创建 Release，也不触发更新。
涉及 UI、音频输出或播放交互的修改，仍须人工检查主界面、播放列表抽屉、搜索、关于和拖拽排序。

测试钩子（环境变量）：`ZZH_OPEN_PLAYLIST=1`、`ZZH_OPEN_SEARCH=1`、`ZZH_OPEN_ABOUT=1`
启动直达对应界面；`ZZH_VERSION_OVERRIDE=x.y.z` 伪装版本号（验证更新流程用）。

发布流程：`Cargo.toml` 版本 → `installer.iss` AppVersion → git tag **三处同步**，
ISCC 打包 → `gh release create vX.Y.Z zzhMusicPlayer_Setup.exe`（走代理）→ 静默覆盖
升级本机 `D:\zzh-music-player`。**禁止在没有用户确认时推送或发版。**

## 架构现状（v1.3.0，阶段 0 拆分后）

### 线程模型

```
UI 线程（Slint 事件循环 + 两个 Timer 泵）
 ├─ 100ms 泵：音频事件 / 文件事件 / 波形结果 / 更新事件 四个 mpsc 的统一消费点
 ├─ 33ms 泵：粒子时钟、主题色补间、工具栏悬停、拖拽排序自动滚动
 ├─ 音频线程（audio_engine.rs）：rodio 播放，Command/Event 通道
 ├─ 波形线程（waveform_generator.rs）：symphonia 流式解码 → 波形/封面/主题色
 └─ 更新线程（updater.rs）：WinINet 查询+下载（代理优先失败转直连）
```

### 文件地图（阶段 0 拆分后）

| 文件 | 职责 |
|---|---|
| `src/state.slint` | 全局状态三域：`TransportState`（主控/传输/主题）、`PlaylistState`（播放列表模块）、`AboutState`（关于与更新模块）；含 `PlaylistEntry`、`UpdateState` |
| `src/widgets.slint` | 通用控件：IconButton（玻璃按钮）、VolumeBar、WaveformArea（波形进度）、PlaylistRow |
| `src/playlist.slint` | 播放列表模块：全窗口抽屉 + 搜索框 + 列表覆盖层（点击/拖拽排序）+ 拖动浮块 |
| `src/about.slint` | 关于与更新模块：遮罩 + 卡片 + GitHub 图标环形下载进度 |
| `src/main.slint` | 主窗口：背景/封面/标题、波形（WaveformArea 实例）、控制胶囊、音量弹层、窗口快捷键与拖动 |
| `src/main.rs` | 组装：全局句柄、回调接线、四个泵、单实例/WndProc/拖拽注册、更新安装 |
| `src/audio_engine.rs` | rodio 封装：Command/Event 通道，播放列表按路径重定位 |
| `src/waveform_generator.rs` | 流式解码聚合波形（min/max/rms）、封面缩略图、中央横带模糊背景、主题色提取（HSV 直方图投票，黑白灰→白色中性） |
| `src/updater.rs` | 纯 WinINet：查 releases/latest、下载安装包、版本比较；系统代理优先失败转直连 |

### 关键设计（为什么是这样的）

- **波形/背景缓存 v4**（`WAVE_CACHE_VERSION`）：每首 ~100KB（160 浮点条 + 128px 封面 +
  324×80 模糊背景），RAM 缓存 8 首、磁盘 50MB LRU。**改任何缓存内容/算法必须 bump
  版本号**，旧缓存按魔数自动失效。
- **播放列表显示模型**（`PlaylistView`）：UI 模型是真实列表过滤后的子集，
  `display_map[display] = real` 映射。**排序/删除后必须全量重建**（`set_vec` 单次重置）——
  Slint 的 `for` 行复用在"删除+插入"成对部分更新下会残留旧行内容（已踩坑：点歌偏移）。
- **主题色**：HSV 色相直方图投票选主导鲜艳色；无彩色封面返回白色中性
  （`NEUTRAL_THEME`）。改算法必须 bump 缓存版本（颜色随缓存持久化）。
- **更新安装**：临时 .cmd（自删除）→ ping 延迟 2 秒 → `/VERYSILENT` 覆盖安装 →
  `start` 重启 `current_exe()`。不要改回 cmd /C 长命令（引号转义会静默失败）。
- **网络**：WinINet `PRECONFIG` 优先（跟随系统代理），失败转 `DIRECT`——共享代理
  出口 IP 的 GitHub 匿名配额（60/h）易耗尽，实测直连可用。

### 已踩过的坑（不要重蹈）

- Slint `Window` 不能同时声明 `height` 和 `min-height`。
- Slint `Path` 的子元素（MoveTo/ArcTo）**直接写在 Path 里**，没有 `elements: []` 数组语法。
- Slint `for` 行复用 + 部分模型更新（remove+insert 对）→ 行内容残留（见上）。
- TextInput 的 `edited`/`accepted` 回调**没有参数**，文本经双向绑定属性读取。
- `windows-sys` 模块名大小写：`Win32_Networking_WinInet`（不是 WinINet）。
- bash heredoc 写长 Python 脚本会被截断——用 Write 工具写脚本文件再执行。
- 管道 `| tail` 会吞掉 cargo 的退出码——判断成败必须 `set -o pipefail`。
- 单实例互斥体 + `FindWindowW("zzhMusicPlayer")` 按窗口标题转发：**模块窗口不得
  使用相同标题**。
- Slint 禁用默认 feature 后无辅助功能树（UIA 枚举不到子元素），GUI 自动化只能走像素。

## 模块化路线图（最终目标 = C）

用户愿景：主面板只是总控；其他功能是可独立关闭的模块；模块可以停靠在主控旁
（接入位置可选），最终可弹出为独立窗口；主控关闭则全部关闭。

**阶段 0（已完成）：状态分域 + 组件拆分**
UIState 大杂烩拆为三个 global；抽屉/关于抽成独立组件；模块组件**窗口无关**
（同一组件既可停靠在主窗口内，也可将来实例化为独立窗口——Slint 全局在同编译
单元的多窗口实例间共享，模块读传输状态零成本）。

**阶段 A（基建已完成）：同窗停靠**
`DockState` 全局 + 主控条最左端"模块"按钮开关停靠区；停靠区固定在主窗口底部
（底部伸展由 Rust 的 `on_dock_changed` 管理窗口高度，保持用户手动调整的基准）；
`test_panel.slint` 的测试面板 A/B 验证滑条与点击交互（无实际功能）。
新模块接入步骤：实现面板组件 → 在停靠区注册标签与 variant → 完成。

**阶段 C（最终）：混合弹出**
在 A 基础上，模块可"弹出为独立无边框窗口"（Slint 多窗口，`set_position` 记忆位置、
吸附主控、逐窗口亚克力与置顶同步、主控关闭逐个收尾）。**按需逐模块开放弹出**，
第一个候选是桌面歌词。基建 3~5 天，只在真需求出现时做。

### 阶段 1~3（功能模块，均以模块化单体形态实现）

1. **音频管线挂钩（已完成）+ 均衡器 UI（待做）**：`audio_engine.rs` 已有
   `DspStage` trait（逐采样 f32 处理）+ `PipelineSource`（解码器→DSP 链→输出，
   无阶段零开销直通）+ `EqStage`（10 段 RBJ peaking，Arc<Mutex<EqSettings>> 共享，
   参数经 Command::SetEq 即时生效）+ 设置持久化（settings.txt `eq=` 行）。
   **剩余：EQ 模块 UI**（停靠面板 10 根滑条 + 预设 + 开关），接 `audio.set_eq()` 即可。
2. **歌词**（1~2 天）：位置事件（50ms）现成；本地 .lrc 解析 → 在线 API（做成可换
   源）；UI 填停靠区或波形上方单行。
3. **自定义音源**（1~2 周，最后做）：`TrackProvider` 抽象（搜索→解析流地址→流式
   喂解码器，HTTP range seek 是难点）；**合规红线：核心不带任何内置源，一律用户
   自行添加**（前车之鉴：LX Music）。若开放第三方，走脚本/进程外 IPC，不走动态库。

## 明确不做

- Rust 动态库插件（abi_stable 路线）——ABI 不稳、崩溃连坐、与自动更新冲突
- slint-interpreter 运行时 UI——体积大、失去类型安全，官方模块用编译期组件
- 异步运行时（tokio）——现有阻塞线程 + mpsc 泵模式够用
- 未经用户确认的 git push / 发版 / 删除用户数据（%APPDATA% 的 settings 是用户数据）

## 性能预算

- 全局拆分是命名空间重组，零运行时成本；**禁止**在 33ms/100ms 泵里做每帧分配
- 泵内属性写入只在值变化时执行（先读后写比对），避免无谓重绘
- 波形/背景纹理保持小图放大方案，禁止引入窗口尺寸的全幅位图（旧版 70-80MB
  内存的历史教训）
