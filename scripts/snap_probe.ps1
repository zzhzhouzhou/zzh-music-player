# One-off smoke: snap linkage verification.
# 1) popout opens snapped right of main  2) move main -> popout follows
# 3) drag popout away >48px -> unsnap     4) drag popout back -> snap again
$ErrorActionPreference = "Stop"
Add-Type -TypeDefinition @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public struct RECT { public int L, T, R, B; }
public class W {
    public delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr l);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder sb, int max);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
    public static IntPtr FindByEnum(string want) {
        IntPtr found = IntPtr.Zero;
        EnumWindows((h, l) => {
            StringBuilder sb = new StringBuilder(128);
            GetWindowTextW(h, sb, 128);
            if (sb.ToString() == want) { found = h; return false; }
            return true;
        }, IntPtr.Zero);
        return found;
    }
}
"@

function Click-At($x, $y) {
    [void][W]::SetCursorPos($x, $y)
    Start-Sleep -Milliseconds 150
    [W]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 60
    [W]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 400
}

# native drag: press at (sx,sy), move in steps, release at (ex,ey)
function Drag-From-To($sx, $sy, $ex, $ey) {
    [void][W]::SetCursorPos($sx, $sy)
    Start-Sleep -Milliseconds 150
    [W]::mouse_event(2, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 120
    $steps = 12
    for ($i = 1; $i -le $steps; $i++) {
        $x = $sx + ($ex - $sx) * $i / $steps
        $y = $sy + ($ey - $sy) * $i / $steps
        [void][W]::SetCursorPos($x, $y)
        Start-Sleep -Milliseconds 25
    }
    Start-Sleep -Milliseconds 150
    [W]::mouse_event(4, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 600
}

$env:ZZH_OPEN_POPOUT = "1"
$errLog = Join-Path $PSScriptRoot "probe_err.log"
if (Test-Path $errLog) { Remove-Item $errLog -Force }
$p = Start-Process -FilePath "target\debug\zzhmusicplayer.exe" -PassThru -RedirectStandardError $errLog
Start-Sleep -Seconds 4
$main = [W]::FindByEnum("zzhMusicPlayer")
$pop = [W]::FindByEnum("zzhMusicPlayer Playlist")
if ($main -eq [IntPtr]::Zero -or $pop -eq [IntPtr]::Zero) { Write-Output "FAIL: windows missing"; Stop-Process -Id $p.Id -Force; exit 1 }
[void][W]::SetForegroundWindow($main)
Start-Sleep -Milliseconds 300

$mr = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr)
$pr0 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr0)
Write-Output ("1. opened: gap={0} dy={1} (snap expected)" -f ($pr0.L - $mr.R), ($pr0.T - $mr.T))

# 2) move main window via its title area drag -> popout must follow
Drag-From-To ($mr.L + 360) ($mr.T + 30) ($mr.L + 360 - 120) ($mr.T + 80)
$mr2 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr2)
$pr2 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr2)
$follows = ($pr2.L - $mr2.R) -eq ($pr0.L - $mr.R) -and ($pr2.T - $mr2.T) -eq ($pr0.T - $mr.T)
Write-Output ("2. main moved ({0},{1}): popout follows = {2} (gap={3})" -f ($mr2.L - $mr.L), ($mr2.T - $mr.T), $follows, ($pr2.L - $mr2.R))

# 3) drag popout far away -> unsnap
Drag-From-To ($pr2.L + 40) ($pr2.T + 20) ($pr2.L + 40 - 300) ($pr2.T + 20 + 200)
$mr3 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr3)
$pr3 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr3)
Write-Output ("3. dragged away: popout at gap={0} dy={1} (expect large = unsnapped)" -f ($pr3.L - $mr3.R), ($pr3.T - $mr3.T))

# 4) drag popout back to right side of main -> re-snap
Drag-From-To ($pr3.L + 40) ($pr3.T + 20) ($mr3.R + 8 + 40) ($mr3.T + 20)
$mr4 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr4)
$pr4 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr4)
# verify snap by moving main again; popout should follow
Drag-From-To ($mr4.L + 360) ($mr4.T + 30) ($mr4.L + 360 + 100) ($mr4.T - 40)
$mr5 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr5)
$pr5 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr5)
$refollows = ($pr5.L - $mr5.R) -eq ($pr4.L - $mr4.R) -and ($pr5.T - $mr5.T) -eq ($pr4.T - $mr4.T)
Write-Output ("4. dragged back + main moved again: popout follows = {0} (gap={1} dy={2})" -f $refollows, ($pr5.L - $mr5.R), ($pr5.T - $mr5.T))

Stop-Process -Id $p.Id -Force
