# zzh-music-player

一款为 Windows 打造的本地音乐播放器。小巧、优雅、美观，用 Rust 和 Slint 写成。

![Rust](https://img.shields.io/badge/Rust-1.85%20-orange?logo=rust)
![Slint](https://img.shields.io/badge/Slint-1.17%20-blue?logo=slint)
![Platform](https://img.shields.io/badge/Platform-Windows%2011%20%7C%2010-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

## 界面预览

![播放界面](screenshots/七月上.png)

![播放列表](screenshots/屏幕截图-播放列表.png)

## 功能

**播放**

支持 MP3、FLAC、WAV、OGG、M4A/AAC。四种播放模式：顺序、列表循环、单曲循环、随机。关闭时会记住播放列表、进度、音量和模式，下次打开接着听。

**封面背景**

自动读取内嵌封面，整张封面高度模糊后作为窗口背景，配合可读性压暗层，任何封面都能保证文字清晰；按钮、波形、高亮统一使用从封面提取的主题色。主色提取按色相直方图投票选出占主导的鲜艳色，黑 / 白 / 灰封面不会再被误判成突兀的亮色，而是使用克制的蓝灰。MP3 的封面由内置的 ID3v2 解析器读取，v2.2 / v2.3 / v2.4 都支持。没有封面的歌曲会隐藏封面区域，并根据文件名生成一个固定不变的专属色调。

**波形进度条**

打开歌曲时在后台解码全部 PCM 数据，生成镜像波形。已播放部分用主题色高亮，播放头平滑滑动；鼠标悬停可以预览任意位置的精确时间，点击或拖动即可跳转。针对响度战音乐（整体压得很响的歌）做了动态范围拉伸，波形不会糊成一片长条。

**播放列表**

展开后覆盖整个窗口的抽屉，按住顶部小条向下滑动即可收起。按住行首手柄拖动排序，浮块跟着鼠标走，拖到列表上下边缘会自动滚动；轻点一行播放，行内有打开所在文件夹和删除按钮。左上角的搜索图标点开即搜，支持正则（非法正则自动退回普通文本匹配，大小写不敏感），回车播放第一首结果；拖入文件夹会在后台扫描并逐批加入列表，大文件夹也不用等。

**细节**

半透明毛玻璃按钮与控制栏；暂停和切歌有音量淡入淡出；空格播放暂停；左右方向键快进快退；Esc 关闭搜索框；滚轮调音量；窗口可置顶；重复打开时文件自动转发给已经运行的实例；关于页面可以直达项目仓库。

## 本地缓存

首次播放会把波形分析结果存到本地缓存，之后再次播放同一首歌，波形、封面、标题直接读取，不需要重新解码。缓存会自动清理，源文件被修改或删除后自动失效。(本地缓存占用极小，理论上，需要保存上千首歌才能达到50MB)

## 安装

到 [Releases](https://github.com/zzhzhouzhou/zzh-music-player/releases) 下载 `zzhMusicPlayer_Setup.exe`，双击安装。安装器会注册 .mp3 / .flac / .wav 的打开方式，之后双击音频文件直接播放。

安装后，程序内的「关于」界面可以检查更新：发现新版本会自动下载（GitHub 图标带环形进度），一键静默升级并自动重启，全程无向导界面。启动后也会静默探测一次新版本，仅在关于按钮上以小圆点提示，不打扰使用。

自己构建：

```
git clone https://github.com/zzhzhouzhou/zzh-music-player.git
cd zzh-music-player
cargo build --release
```

构建产物在 `target\release\zzhmusicplayer.exe`。需要安装包的话，用 Inno Setup 编译 `installer.iss` 即可。

## 技术栈

Rust · Slint · rodio · symphonia。音频解码和波形分析都在后台线程完成，界面线程只负责渲染。

## 许可证

[MIT](LICENSE)
