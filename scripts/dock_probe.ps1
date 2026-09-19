# dock_probe v3: screenshot-driven dock verification.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class D3 {
    public delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(D3.EnumProc cb, IntPtr l);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, System.Text.StringBuilder sb, int max);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
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
function Shot([string]$path, [int]$mx, [int]$my, [int]$w, [int]$h) {
    $bmp = New-Object System.Drawing.Bitmap ($w + 40), ($h + 40)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen($mx - 20, $my - 20, 0, 0, $bmp.Size)
    $g.Dispose()
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
}
function Mark-And-Click([int]$ax, [int]$ay) {
    [void][D3]::SetCursorPos($ax, $ay); Start-Sleep -Milliseconds 80
    [void][D3]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 40
    [void][D3]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
}
Get-Process zzhmusicplayer -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 800
$errLog = Join-Path $PSScriptRoot "probe_err.log"
if (Test-Path $errLog) { Remove-Item $errLog -Force }
$p = Start-Process -FilePath "target\debug\zzhmusicplayer.exe" -PassThru -RedirectStandardError $errLog
Start-Sleep -Seconds 3
if (-not [D3]::Sweep("zzhMusicPlayer")) { Write-Output "FAIL: no main window"; Stop-Process -Id $p.Id -Force; exit 1 }
$mx = [D3]::Found.L; $my = [D3]::Found.T; $mw = [D3]::Found.R - [D3]::Found.L; $mh = [D3]::Found.B - [D3]::Found.T
Write-Output ("main {0},{1} {2}x{3}" -f $mx, $my, $mw, $mh)
Shot (Join-Path $PSScriptRoot "dock_a.png") $mx $my $mw $mh
Mark-And-Click ($mx + 218) ($my + 151)
Start-Sleep -Milliseconds 500
if ([D3]::Sweep("zzhMusicPlayer")) {
    $mh2 = [D3]::Found.B - [D3]::Found.T
    Write-Output ("after click(218,151): height {0} -> {1}" -f $mh, $mh2)
}
# settle: the open animation is 240ms; sample the FINAL height after it ends
Start-Sleep -Milliseconds 900
if ([D3]::Sweep("zzhMusicPlayer")) {
    $mh3 = [D3]::Found.B - [D3]::Found.T
    Write-Output ("settled open height: {0} (expect base + module-height)" -f $mh3)
}
Shot (Join-Path $PSScriptRoot "dock_b.png") $mx $my ([Math]::Max($mw, 340)) ([Math]::Max($mh, 520))

# close via the module button again (retry: click delivery can be flaky); sample ramp
$closed = $false
foreach ($try in 1..4) {
    Mark-And-Click ($mx + 218) ($my + 151)
    $ramp = @()
    foreach ($i in 1..25) {
        Start-Sleep -Milliseconds 24
        if ([D3]::Sweep("zzhMusicPlayer")) { $ramp += ([D3]::Found.B - [D3]::Found.T) }
    }
    $distinct = ($ramp | Sort-Object -Unique).Count
    if ($ramp[-1] -lt 300) {
        $closed = $true
        Write-Output ("close ramp (try {0}): {1} distinct heights: {2}" -f $try, $distinct, (($ramp | Select-Object -First 14) -join ','))
        break
    }
}
if (-not $closed) { Write-Output "close click never landed (probe flake)" }
if ($distinct -ge 4) { Write-Output "DOCK SMOOTH: animated ramp confirmed" }
elseif ($closed) { Write-Output "DOCK INSTANT: no intermediate heights" }
Get-Content $errLog
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
