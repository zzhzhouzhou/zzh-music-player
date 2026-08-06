# zzh-music-player

> 一款简洁、轻量、纯粹的音频播放器

![Rust](https://img.shields.io/badge/Rust-1.85%20-orange?logo=rust)
![Slint](https://img.shields.io/badge/Slint-1.7%20-blue?logo=slint)
![Platform](https://img.shields.io/badge/Platform-Windows%2011%20%7C%2010-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

---

## ## 🖼️ 界面预览

![播放器截图](file:///E:/zzhMusicPlayer/screenshots/%E6%B5%8B%E8%AF%95%E9%9F%B3%E9%A2%91.png)

![截图展示](file:///E:/zzhMusicPlayer/screenshots/%E8%8B%A5%E6%8A%8A%E4%BD%A0.png)

## 特性

- **核心播放**  
  支持 MP3、FLAC、WAV、OGG、M4A/AAC 等常见格式。

- **智能视觉**  
  有封面时提取主题色作为渐变背景，无封面时根据文件名哈希生成专属色调。

- **波形进度条**
  实时显示音频 PCM 波形。

- **播放列表**  
  支持拖拽添加、列表循环 / 单曲循环 / 随机播放。列表自动持久化。

- **窗口置顶**  
  窗口置顶，随时调整，不受突然窗口影响

- **记忆播放**  
  自动保存当前播放列表、播放进度、音量和播放模式。下次打开，接着听。

- **双击文件关联**  
  在系统设置中设为默认音乐播放器后，双击音频文件即可直接播放。

---

## 📦 安装与使用

### 下载预编译版本（推荐）

前往 [Releases](https://github.com/zzhzhouzhou/zzh-music-player/releases) 页面下载最新 `.exe` 文件，双击即可运行。



# 克隆仓库

```
git clone https://github.com/zzhzhouzhou/zzh-music-player.git
cd zzh-music-player
```



# 构建发布版本

```
cargo build --release
```

# 运行

```
./target/release/zzh-music-player.exe
```
