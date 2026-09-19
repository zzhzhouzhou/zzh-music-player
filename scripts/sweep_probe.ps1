# sweep_probe v2: find the true x of the bar playlist button by sweeping clicks
# and watching for the "playlist-closed" ASCII marker in stderr. ASCII only.
$ErrorActionPreference = 'Stop'
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class W2 {
    public delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(W2.EnumProc cb, IntPtr l);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, System.Text.StringBuilder sb, int max);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
    public static RECT Found;
    public static bool Sweep(string title) {
        bool ok = false;
        EnumWindows(delegate(IntPtr h, IntPtr l) {
            var sb = new System.Text.StringBuilder(256);
            GetWindowTextW(h, sb, 256);
            if (sb.ToString() == title) { RECT r; GetWindowRect(h, out r); Found = r; ok = true; return false; }
            return true;
        }, IntPtr.Zero);
        return ok;
    }
}
"@
Get-Process zzhmusicplayer -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 800
$env:ZZH_OPEN_POPOUT = "1"
$errLog = Join-Path $PSScriptRoot "probe_err.log"
if (Test-Path $errLog) { Remove-Item $errLog -Force }
$p = Start-Process -FilePath "target\debug\zzhmusicplayer.exe" -PassThru -RedirectStandardError $errLog
Start-Sleep -Seconds 4
if (-not [W2]::Sweep("zzhMusicPlayer")) { Write-Output "main not found"; Stop-Process -Id $p.Id -Force; exit 1 }
$mx = [W2]::Found.L; $my = [W2]::Found.T
Write-Output ("main at {0},{1}" -f $mx, $my)
$baseline = (Get-Content $errLog).Count

$hit = -1
foreach ($x in 430..545) {
    if ($x % 5 -ne 0) { continue }
    [void][W2]::SetCursorPos($mx + $x, $my + 151); Start-Sleep -Milliseconds 90
    [W2]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40
    [W2]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 600
    $log = Get-Content $errLog -Raw
    if ($log.Contains("playlist-closed")) { $hit = $x; break }
}
Write-Output ("playlist-closed triggered at x offset = {0} (bar y=151)" -f $hit)
Get-Content $errLog
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
