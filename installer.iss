[Setup]
AppId={{2E05B647-1616-4F5F-A951-5DE650895527}}   ; 固定此 GUID，所有版本保持一致
AppName=zzh音乐播放器
AppVersion=1.1.0
AppPublisher=zzhzhouzhou
DefaultDirName={autopf}\zzhMusicPlayer
DefaultGroupName=zzh音乐播放器
UninstallDisplayIcon={app}\zzhmusicplayer.exe
Compression=lzma2
SolidCompression=yes
OutputDir=.
OutputBaseFilename=zzhMusicPlayer_Setup

; 安装程序图标
SetupIconFile=icons\appicon\zzhmp.ico

; 推荐加上这两行，提升升级体验（可选）
; CloseApplications=yes
; UsePreviousAppDir=yes

[Files]
Source: "target\release\zzhmusicplayer.exe"; DestDir: "{app}"
Source: "icons\appicon\zzhmp.ico"; DestDir: "{app}"

[Icons]
Name: "{group}\zzh音乐播放器"; Filename: "{app}\zzhmusicplayer.exe"; IconFilename: "{app}\zzhmp.ico"
Name: "{commondesktop}\zzh音乐播放器"; Filename: "{app}\zzhmusicplayer.exe"; IconFilename: "{app}\zzhmp.ico"

[Run]
Filename: "{app}\zzhmusicplayer.exe"; Description: "启动 zzh音乐播放器"; Flags: postinstall nowait skipifsilent

[Registry]
; 改为 HKCU\Software\Classes，避免需要管理员权限（兼容 winget）
Root: HKCU; Subkey: "Software\Classes\.mp3"; ValueType: string; ValueData: "zzhMusicPlayer.mp3"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.flac"; ValueType: string; ValueData: "zzhMusicPlayer.flac"; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.wav"; ValueType: string; ValueData: "zzhMusicPlayer.wav"; Flags: uninsdeletevalue

Root: HKCU; Subkey: "Software\Classes\zzhMusicPlayer.mp3\DefaultIcon"; ValueType: string; ValueData: "{app}\zzhmp.ico"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\zzhMusicPlayer.mp3\Shell\Open\Command"; ValueType: string; ValueData: """{app}\zzhmusicplayer.exe"" ""%1"""; Flags: uninsdeletekey

Root: HKCU; Subkey: "Software\Classes\zzhMusicPlayer.flac\DefaultIcon"; ValueType: string; ValueData: "{app}\zzhmp.ico"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\zzhMusicPlayer.flac\Shell\Open\Command"; ValueType: string; ValueData: """{app}\zzhmusicplayer.exe"" ""%1"""; Flags: uninsdeletekey

Root: HKCU; Subkey: "Software\Classes\zzhMusicPlayer.wav\DefaultIcon"; ValueType: string; ValueData: "{app}\zzhmp.ico"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\zzhMusicPlayer.wav\Shell\Open\Command"; ValueType: string; ValueData: """{app}\zzhmusicplayer.exe"" ""%1"""; Flags: uninsdeletekey