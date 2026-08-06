[Setup]
AppName=zzh音乐播放器
AppVersion=1.0.0
AppPublisher=zzhzhouzhou
DefaultDirName={pf}\zzhMusicPlayer
DefaultGroupName=zzh音乐播放器
UninstallDisplayIcon={app}\zzhmusicplayer.exe
Compression=lzma2
SolidCompression=yes
OutputDir=.
OutputBaseFilename=zzhMusicPlayer_Setup

; ===== 新增：安装程序本身的图标 =====
SetupIconFile=icons\appicon\zzhmp.ico

[Files]
; 可执行文件
Source: "target\release\zzhmusicplayer.exe"; DestDir: "{app}"
; ===== 新增：把图标文件复制到安装目录 =====
Source: "icons\appicon\zzhmp.ico"; DestDir: "{app}"

[Icons]
; 开始菜单快捷方式（指定图标）
Name: "{group}\zzh音乐播放器"; Filename: "{app}\zzhmusicplayer.exe"; IconFilename: "{app}\zzhmp.ico"
; 桌面快捷方式（指定图标）
Name: "{commondesktop}\zzh音乐播放器"; Filename: "{app}\zzhmusicplayer.exe"; IconFilename: "{app}\zzhmp.ico"

[Run]
; 安装完成后可选运行程序
Filename: "{app}\zzhmusicplayer.exe"; Description: "启动 zzh音乐播放器"; Flags: postinstall nowait skipifsilent

; ===== 关键：在注册表中建立文件关联 =====
[Registry]
; 让 .mp3 等后缀关联到你的程序
Root: HKCR; Subkey: ".mp3"; ValueType: string; ValueData: "zzhMusicPlayer.mp3"; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".flac"; ValueType: string; ValueData: "zzhMusicPlayer.flac"; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".wav"; ValueType: string; ValueData: "zzhMusicPlayer.wav"; Flags: uninsdeletevalue
; 其他格式按需添加...

; 定义打开方式 (Shell\Open\Command)
Root: HKCR; Subkey: "zzhMusicPlayer.mp3\DefaultIcon"; ValueType: string; ValueData: "{app}\zzhmp.ico"; Flags: uninsdeletekey
Root: HKCR; Subkey: "zzhMusicPlayer.mp3\Shell\Open\Command"; ValueType: string; ValueData: """{app}\zzhmusicplayer.exe"" ""%1"""; Flags: uninsdeletekey

Root: HKCR; Subkey: "zzhMusicPlayer.flac\DefaultIcon"; ValueType: string; ValueData: "{app}\zzhmp.ico"; Flags: uninsdeletekey
Root: HKCR; Subkey: "zzhMusicPlayer.flac\Shell\Open\Command"; ValueType: string; ValueData: """{app}\zzhmusicplayer.exe"" ""%1"""; Flags: uninsdeletekey

Root: HKCR; Subkey: "zzhMusicPlayer.wav\DefaultIcon"; ValueType: string; ValueData: "{app}\zzhmp.ico"; Flags: uninsdeletekey
Root: HKCR; Subkey: "zzhMusicPlayer.wav\Shell\Open\Command"; ValueType: string; ValueData: """{app}\zzhmusicplayer.exe"" ""%1"""; Flags: uninsdeletekey