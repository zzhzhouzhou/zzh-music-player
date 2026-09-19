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
3. **功能不变原则** — 任何重构/新功能合并前，26 个单元测试必须全绿，且全部 UI 面
   （主界面 / 播放列表抽屉 / 搜索 / 关于 / 拖拽排序 / 播放列表弹窗）人工过一遍。
4. 中文注释，注释解释 **为什么** 而不是做了什么。

## 构建与验证

```bash
cargo test                  # 26 个单元测试（波形/ID3/更新器/缓存/搜索过滤/DSP管线/seek回归）
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
**用户固定要求：每次改动完成并通过检查后，必须同时构建 release 并静默覆盖安装本机**
（`cargo build --release` → ISCC 打包 → 安装包 `/VERYSILENT` 覆盖升级 `D:\zzh-music-player`），
只跑 debug 验证不算完成——用户日常运行的是安装版。

测试钩子（环境变量）：`ZZH_OPEN_PLAYLIST=1`、`ZZH_OPEN_SEARCH=1`、`ZZH_OPEN_ABOUT=1`、
`ZZH_OPEN_POPOUT=1` 启动直达对应界面；`ZZH_VERSION_OVERRIDE=x.y.z` 伪装版本号
（验证更新流程用）。

GUI 回归探针（PowerShell，ASCII-only，SendInput 驱动 + SendMessageTimeout
存活检测；点击前必须 SetForegroundWindow + 中性位置激活点击，非前台窗口的
首次点击可能只激活不投递）：
- `scripts/drag_probe.ps1 [exe]` — 启动直达弹窗，30 轮原生拖拽压力 + 每轮
  存活检测 + 末尾拖拽位移验证；压测中 `[pop-drag] skip` 表示守卫拦下误触发。
- `scripts/popout_probe.ps1 [empty]` — 真实路径（抽屉→⤴→弹窗）弹出/收回/
  再弹出计时；`empty` 参数用干净 APPDATA 隔离字形量成本。
- `scripts/dock_probe.ps1` — 模块按钮开停靠区，采样窗口高度渐变验证动画。
- `scripts/liveness_probe.ps1` — 启动响应时间曲线（首次响应毫秒数）。
- `scripts/sweep_probe.ps1` — 沿 x 扫掠 + 日志断言，定位控件真实坐标。

发布流程：`Cargo.toml` 版本 → `installer.iss` AppVersion → git tag **三处同步**，
ISCC 打包 → `gh release create vX.Y.Z zzhMusicPlayer_Setup.exe`（走代理）→ 静默覆盖
升级本机 `D:\zzh-music-player`。**禁止在没有用户确认时推送或发版。**

## 架构现状（v1.3.0，阶段 0 拆分后）

### 线程模型

```
UI 线程（Slint 事件循环 + 两个 Timer 泵）
 ├─ 16ms 事件泵：音频事件 / 文件事件 / 波形结果 / 更新事件 四个 mpsc 的统一消费点
 ├─ 33ms 泵：粒子时钟、主题色补间、工具栏悬停、拖拽排序自动滚动、弹窗属性桥接
 ├─ 音频线程（audio_engine.rs）：rodio 播放，Command/Event 通道
 ├─ 波形线程（waveform_generator.rs）：symphonia 流式解码 → 波形/封面/主题色
 └─ 更新线程（updater.rs）：WinINet 查询+下载（代理优先失败转直连）
```

### 文件地图（组合根拆分后：main.rs 薄入口 + app.rs 组装）

| 文件 | 职责 |
|---|---|
| `src/state.slint` | 全局状态分域：`TransportState`（主控/传输/主题）、`PlaylistState`（播放列表模块）、`AboutState`（关于与更新模块）、`DockState`（停靠区）、`EqState`（均衡器模块）；含 `PlaylistEntry`、`UpdateState` |
| `src/widgets.slint` | 通用控件：IconButton（玻璃按钮）、VolumeBar、WaveformArea（波形进度）、PlaylistRow |
| `src/playlist.slint` | 播放列表面板（`PlaylistPanel`，窗口无关）+ 抽屉外壳（`PlaylistDrawer`：开合动画 + 弹出入口）+ 搜索框 + 列表覆盖层（点击/拖拽排序）+ 拖动浮块 |
| `src/playlist_window.slint` | 播放列表独立窗口（阶段 C）：无边框亚克力小窗，复用 `PlaylistPanel`；标题不得与主窗口相同 |
| `src/about.slint` | 关于与更新模块：遮罩 + 卡片 + GitHub 图标环形下载进度 |
| `src/eq.slint` | 均衡器面板（阶段 1）：10 段竖向滑条 ±12dB（0dB 中线双向填充）+ 6 预设 + 开关；窗口无关组件 |
| `src/main.slint` | 主窗口：背景/封面/标题、波形（WaveformArea 实例）、控制胶囊、音量弹层、窗口快捷键与拖动、停靠区（均衡器） |
| `src/main.rs` | 模块树根 + 薄入口：`slint::include_modules!()` 的生成类型落在这里（crate 根），子模块经 `crate::` 引用；实际逻辑只有 `app::run()` |
| `src/app.rs` | 组合根：`App` 结构（全部共享状态集中一处）、`run()` 装配（窗口/引擎/通道/计时器/设置恢复/启动参数）、`do_close` 退出持久化、弹窗开关（`open/close_playlist_window`） |
| `src/ui_callbacks.rs` | Slint 回调接线，按四域分组注册：传输 / 播放列表 / 关于与更新 / 窗口壳层（拖动 + 双击）；播放列表动作经 `action_*` 共享函数供主窗抽屉与弹窗复用；闭包只持 `Weak<App>` |
| `src/pumps.rs` | 两只周期泵的泵体：16ms 事件泵四通道统一消费点（音频/文件/波形/更新，轮询顺序勿改），33ms 粒子/悬停/自动滚动 + 弹窗属性桥接同步 |
| `src/events.rs` | 跨线程事件类型与后台生产者：`FileEvent`/`UpdateEvent`、更新检查/下载线程、文件夹递归扫描线程 |
| `src/playlist.rs` | 播放列表域：`PlaylistView` 显示模型（过滤 + 显示行→真实行映射）、`Matcher` 搜索、增删播业务操作；搜索/映射测试在此 |
| `src/waveform.rs` | 波形域：`WaveformResult`、后台工作线程（最新任务优先 + 代次取消）、RAM 缓存写入（LRU）、`apply_waveform` 上屏 |
| `src/transport.rs` | 传输域纯 Rust 状态：`SeekState` 跳转回执过滤、`ThemeTween` 主题色 HSL 补间 |
| `src/render_utils.rs` | 渲染小工具：兜底背景位图（80×48 光斑）、占位波形、m:ss 时间文本、背景交叉淡入 |
| `src/version.rs` | `app_version()`：单一来源 Cargo.toml；`ZZH_VERSION_OVERRIDE` 测试覆盖 |
| `src/audio_engine.rs` | rodio 封装：Command/Event 通道，播放列表按路径重定位；DspStage/EqStage 音频管线 |
| `src/waveform_generator.rs` | 流式解码聚合波形（min/max/rms）、封面缩略图、中央横带模糊背景、主题色提取（HSV 直方图投票，黑白灰→白色中性） |
| `src/waveform_cache.rs` | 波形/封面/背景磁盘缓存 v4：50MB LRU、魔数版本失效 |
| `src/settings.rs` | settings.txt 行式读写：音量/模式/EQ/播放列表/置顶/上次播放/弹窗位置（pop-x/pop-y） |
| `src/updater.rs` | 纯 WinINet：查 releases/latest、下载安装包、版本比较；系统代理优先失败转直连 |
| `src/windows_platform.rs` | Win32 集成：亚克力/圆角、单例互斥 + WM_COPYDATA 转发、WndProc 子类化（主窗：拖拽/滚轮/WM_CLOSE；弹窗：WM_CLOSE 收回） |

### 关键设计（为什么是这样的）

- **波形/背景缓存 v4**（`WAVE_CACHE_VERSION`）：每首 ~100KB（160 浮点条 + 128px 封面 +
  324×80 模糊背景），RAM 缓存 8 首、磁盘 50MB LRU。**改任何缓存内容/算法必须 bump
  版本号**，旧缓存按魔数自动失效。
- **播放列表显示模型**（`PlaylistView`）：UI 模型是真实列表过滤后的子集，
  `display_map[display] = real` 映射。**排序/删除后必须全量重建**（`set_vec` 单次重置）——
  Slint 的 `for` 行复用在"删除+插入"成对部分更新下会残留旧行内容（已踩坑：点歌偏移）。
- **主题色**：HSV 色相直方图投票选主导鲜艳色；无彩色封面返回白色中性
  （`NEUTRAL_THEME`）。改算法必须 bump 缓存版本（颜色随缓存持久化）。
- **播放列表独立弹窗（阶段 C 首落地）**：`PlaylistWindow` 与主窗抽屉共用
  `PlaylistPanel`。**Slint 全局按组件实例隔离**（slint 1.17 文档明确），弹窗拿到的
  是另一份全局——跨窗口桥接只有两条路：列表内容共享同一 `ModelRc`（天然同步），
  其余轻量属性（主题色/播放状态/粒子时钟/当前曲目）由泵从主窗实例单向同步
  （仅变化时写入）。弹窗即建即毁（关闭即 drop 释放资源），每次弹出固定在
  主窗口右侧 +8px（右缘放不下翻左侧，纵向贴顶缘并夹回屏内；不做绝对位置
  记忆），主控关闭级联收回。弹窗动作回调与主窗共用
  `action_*` 实现（ui_callbacks.rs）。
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
- rodio 的 `Source::try_seek` 带默认实现（返回 `NotSupported`）：**包装解码器的
  自定义 Source 必须转发 try_seek**，否则所有跳转静默失败（阶段 1 引入
  PipelineSource 时踩过，已有回归测试 `pipeline_forwards_seek_to_inner`）。
- Slint 全局单例**按组件实例隔离**（每个导出组件实例一套全局）——多窗口之间
  属性不互通，跨窗口状态必须 Rust 侧桥接（见"播放列表独立弹窗"设计）。
- Slint 窗口 height 直接加动画会产生"先瞬移再滑回"的布局瞬态（依赖
  `height - X` 定位的元素会瞬移出窗）。**安全模式是共享镜像动画**（停靠区
  开合的现行做法）：`property <length> dock-h: open ? mh : 0px; animate dock-h`
  然后 `height = base + dock-h`、`bar-y = height - 46 - dock-h`——所有依赖项
  共用同一动画值，差值在逐帧动画期间数学恒定，主控条纹丝不动；容器 clip +
  内部内容尺寸引用 module-height（不压缩，幕布式揭开）。探针
  `scripts/dock_probe.ps1` 可采样中间高度验证渐变。
- **次级窗口拖动必须走系统原生拖动**（`begin_native_drag`：ReleaseCapture +
  `WM_NCLBUTTONDOWN`/HTCAPTION），不要用 GetCursorPos 增量自算位置——混合 DPI /
  远程会话下系统光标读数与 Slint 坐标空间存在漂移偏移，自算会让窗口跑离光标、
  拖动数帧后中断（主窗口全窗 TouchArea 因窗口 1:1 跟随光标而未暴露此问题）。
  弹窗拖拽可用 `scripts/drag_probe.ps1` 做回归验证。
- **原生拖动两个防御缺一不可**（"弹窗偶尔假死"的根因，30 轮压测实测拦到
  2 次误触发）：① `GetAsyncKeyState(VK_LBUTTON)` 确认左键确实按住才进入——
  右键/合成双击事件误触发时模态移动循环等不到左键释放，UI 线程无限卡死；
  ② 用 `PostMessageW` 而非 `SendMessageW`——模态循环从消息泵顶层启动，
  不在 Slint 回调栈内嵌套分发消息（winit/Slint 不保证可重入）。
- **Slint 事件循环对"显示中"的窗口持强引用**：关闭次级窗口必须先
  `pw.hide()` 再 drop，否则留下幽灵窗口——仍可见但脱离管理（✕ 只会去开
  抽屉、Alt+F4 落在空引用上），即"弹窗像死了一样无法关闭"。探针
  `scripts/popout_probe.ps1` 实测：drop 前无 hide 时收回后窗口存活 >10s。
- **femtovg 每窗口一个 GL 上下文 => 字形缓存按窗口实例独立**：次级窗口声明
  大字符集字体（HarmonyOS Sans SC）会在每次窗口创建+销毁时于 UI 线程光栅化
  整张列表的 CJK 字形，~100 首实测弹出 3.8s + 收回再 3.8s 全程冻结（空列表仅
  652ms，随列表规模线性）。因此弹窗（`playlist_window.slint`）**不声明**
  `default-font-family`（系统回退字体，瞬时打开）；主窗/抽屉共享主窗上下文，
  用 HarmonyOS Sans SC（标题 font-weight 500 = Medium）。字体一致性若将来
  必须，可考虑持久化弹窗实例（hide/show 代替即建即毁）——接受内存代价。
- **Slint 入口文件必须显式 `export { ... } from "xxx.slint"` 重导出全局**，`include_modules!()`
  才会在 crate 根生成对应类型——只在入口 `import`（供组件使用）不会生成
  `crate::EqState` 这类类型（E0432 且报错不指向真正原因）。
- **次级窗口的 set_position 必须等 winit 窗口真正创建后调用**：`show()` 返回时
  HWND 仍可能未就绪（winit 惰性创建），此前的 set_position 被**静默丢弃**，
  窗口落在 winit 默认位置（实测：定位写在 show 后仍被丢，挂进现有
  "HWND 未就绪则 50ms 重试"分支后才生效）。位置计算统一走 Win32
  `GetWindowRect`（物理像素）——Slint 的 position/size 语义在窗口创建前后
  有出入，勿混用逻辑/物理单位。
- **泵周期更新的属性挂 Slint `animate` = 播放中动画时钟常驻**（曾致
  240Hz 屏 237fps、40% 单核）：补间时长 > 泵写入间隔意味着动画永远在途，
  winit 以显示器刷新率持续整窗重绘。**处理原则（用户拍板）：美观与性能
  并重**——在性能预算允许的前提下极致追求美观；粒子 opacity（120ms）与
  播放头 played-frac（150ms linear）补间已恢复，接受"仅播放中"的重绘代价
  （暂停/空闲 position 不更新，动画自然沉降回惰性渲染）。新属性仍须遵守：
  能不挂就不挂；确要挂则用短补间（≤150ms）并确认代价只发生在交互/播放期，
  禁止在纯空闲路径引入常驻动画。
  探针方法：TotalProcessorTime 双采样 + SLINT_DEBUG_PERFORMANCE=refresh_lazy,console 真实 fps。
- **弹窗磁贴联动（snap）**：贴靠态在 33ms 泵里每拍 `GetWindowRect` 对齐
  主窗（主窗动 → 弹窗跟随），拖离 >48px 解绑、拖回主窗左右 24px 贴靠带
  吸上；左键按住期间整段跳过（原生拖动是模态循环，泵不能抢窗口位置）。
  状态在 `App.pop_snap / pop_snap_off`，弹出默认贴靠右侧。
- **UI 线程看门狗**（pumps.rs `spawn_ui_watchdog` + `PUMP_TICK`）：33ms 泵
  心跳停止 >1s 即输出 `[watchdog]` 诊断，是排查"假死"类问题的第一现场；
  后台线程零锁零分配，勿在泵内加锁。
- 关键生命周期日志带 ASCII 尾标（`playlist-opened` / `playlist-closed`），
  GBK 控制台下可直接 grep——GUI 自动化探针靠它断言状态。

## 模块化路线图（最终目标 = C）

用户愿景：主面板只是总控；其他功能是可独立关闭的模块；模块可以停靠在主控旁
（接入位置可选），最终可弹出为独立窗口；主控关闭则全部关闭。

**阶段 0（已完成）：状态分域 + 组件拆分**
UIState 大杂烩拆为三个 global；抽屉/关于抽成独立组件；模块组件**窗口无关**
（同一组件既可停靠在主窗口内，也可实例化为独立窗口。注意：Slint 全局按组件
实例隔离，多窗口并不天然共享全局——跨窗口同步靠 Rust 侧桥接，见关键设计）。

**阶段 A（基建已完成）：同窗停靠**
`DockState` 全局 + 主控条最左端"模块"按钮开关停靠区；停靠区固定在主窗口底部
（开合为 240ms 幕布式平滑动画：动画驱动源是 `dock-h` 共享镜像，窗口高度与
主控条 y 共用同一动画值，差值恒定无布局瞬态，见坑列表）。首 个真实模块
**均衡器**（`eq.slint`）已停靠；播放列表抽屉打开时覆盖整个窗口（含停靠区）。
新模块接入步骤：实现面板组件 → 在停靠区注册标签与内容。

**阶段 C（首落地）：混合弹出**
播放列表已可弹出为独立无边框窗口（弹出/收回切换 + 固定贴主窗右侧 + 亚克力/圆角 +
主控关闭级联收回；暂无吸附）。基建已通：Slint 多窗口 + 逐窗口系统效果 +
独立 WndProc 子类化 + 泵桥接同步。**按需逐模块开放弹出**，下一个候选是
桌面歌词；新模块弹出照抄播放列表的模式（面板组件窗口无关 + 共享模型 + 泵同步）。

### 阶段 1~3（功能模块，均以模块化单体形态实现）

1. **音频管线挂钩（已完成）+ 均衡器 UI（已完成）**：`audio_engine.rs` 已有
   `DspStage` trait（逐采样 f32 处理）+ `PipelineSource`（解码器→DSP 链→输出，
   无阶段零开销直通；**必须转发 try_seek**，见坑列表）+ `EqStage`（10 段 RBJ
   peaking，Arc<Mutex<EqSettings>> 共享，参数经 Command::SetEq 即时生效）+
   设置持久化（settings.txt `eq=` 行）。
   EQ UI（`eq.slint` + `EqState`）：停靠面板 10 根滑条 + 6 预设（平直/流行/
   摇滚/爵士/人声/古典）+ 开关；增益单一数据源是 `app.eq_gains_model`
   （VecModel<f32>），每次改动重建 EqSettings 下发引擎并同步 `app.eq`
   （RefCell，do_close 持久化）；手动拖动任一滑条即落回"自定义"。
2. **歌词**（1~2 天）：位置事件（50ms）现成；本地 .lrc 解析 → 在线 API（做成可换
   源）；UI 填停靠区或波形上方单行。
3. **自定义音源（暂缓）**：用户已决定推迟，不排入当前计划；届时从
   `TrackProvider` 抽象做起（搜索→解析流地址→流式喂解码器，HTTP range seek
   是难点）；**合规红线：核心不带任何内置源，一律用户自行添加**（前车之鉴：
   LX Music）。若开放第三方，走脚本/进程外 IPC，不走动态库。

## 明确不做

- Rust 动态库插件（abi_stable 路线）——ABI 不稳、崩溃连坐、与自动更新冲突
- slint-interpreter 运行时 UI——体积大、失去类型安全，官方模块用编译期组件
- 异步运行时（tokio）——现有阻塞线程 + mpsc 泵模式够用
- 未经用户确认的 git push / 发版 / 删除用户数据（%APPDATA% 的 settings 是用户数据）

## 性能预算

- 全局拆分是命名空间重组，零运行时成本；**禁止**在 33ms/16ms 泵里做每帧分配
- 泵内属性写入只在值变化时执行（先读后写比对），避免无谓重绘
- 波形/背景纹理保持小图放大方案，禁止引入窗口尺寸的全幅位图（旧版 70-80MB
  内存的历史教训）
