[Setup]
AppName=zzh音乐播放器
AppVersion=0.1.0
AppPublisher=zzhzhouzhou
DefaultDirName={pf}\zzhMusicPlayer
DefaultGroupName=zzh音乐播放器
UninstallDisplayIcon={app}\zzhmusicplayer.exe
Compression=lzma2
SolidCompression=yes
OutputDir=.
OutputBaseFilename=zzhMusicPlayer_Setup

[Files]
; 注意：可执行文件的路径必须与你的编译输出一致
Source: "target\release\zzhmusicplayer.exe"; DestDir: "{app}"

[Icons]
; 开始菜单快捷方式
Name: "{group}\zzh音乐播放器"; Filename: "{app}\zzhmusicplayer.exe"
; 桌面快捷方式
Name: "{commondesktop}\zzh音乐播放器"; Filename: "{app}\zzhmusicplayer.exe"

[Run]
; 安装完成后可选运行程序
Filename: "{app}\zzhmusicplayer.exe"; Description: "启动 zzh音乐播放器"; Flags: postinstall nowait skipifsilent