# Snap (tile-linkage) regression probe.
# Order matters: the unsnap drag must run right after open (popup certainly
# has focus then; a drag later in the sequence — after the main window's own
# native drag — can be swallowed by the activation-only first click, an
# automation artifact, not an app bug).
# 1) popup opens snapped right of main
# 2) drag popup away >48px -> unsnap (pump stops following)
# 3) drag popup back into the snap band -> re-snap
# 4) move main -> popup follows (real-time, same message loop)
# 5) FAST fling of main -> still snapped (the user-reported bug: fast drags
#    used to open a gap and unsnap)
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
$gap0 = $pr0.L - $mr.R
Write-Output ("1. opened: gap={0} dy={1} (snap expected)" -f $gap0, ($pr0.T - $mr.T))

# 2) drag popup away immediately (popup has focus right after open) -> unsnap
Drag-From-To ($pr0.L + 40) ($pr0.T + 20) ($pr0.L + 40 - 300) ($pr0.T + 20 + 200)
$mr2 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr2)
$pr2 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr2)
$dx2 = [Math]::Abs(($pr2.L - $mr2.R) - $gap0)
Write-Output ("2. dragged away: popout at gap={0} dy={1} (expect large = unsnapped)" -f ($pr2.L - $mr2.R), ($pr2.T - $mr2.T))

# 3) drag popup back into the right snap band -> re-snap
Drag-From-To ($pr2.L + 40) ($pr2.T + 20) ($mr2.R + 8 + 40) ($mr2.T + 20)
$mr3 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr3)
$pr3 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr3)
$gap3 = $pr3.L - $mr3.R
Write-Output ("3. dragged back: gap={0} dy={1} (expect ~{2} = re-snapped)" -f $gap3, ($pr3.T - $mr3.T), $gap0)

# 4) move main via its title area drag -> popup must follow (real-time)
Drag-From-To ($mr3.L + 360) ($mr3.T + 30) ($mr3.L + 360 - 120) ($mr3.T + 80)
$mr4 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr4)
$pr4 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr4)
$follows = [Math]::Abs(($pr4.L - $mr4.R) - $gap3) -le 24 -and [Math]::Abs(($pr4.T - $mr4.T) - ($pr3.T - $mr3.T)) -le 24
Write-Output ("4. main moved: popout follows = {0} (gap={1})" -f $follows, ($pr4.L - $mr4.R))

# 5) FAST fling of main (few large steps) -> must stay snapped
Drag-From-To ($mr4.L + 360) ($mr4.T + 30) ($mr4.L + 360 + 500) ($mr4.T - 200)
$mr5 = New-Object RECT; [void][W]::GetWindowRect($main, [ref]$mr5)
$pr5 = New-Object RECT; [void][W]::GetWindowRect($pop, [ref]$pr5)
$fastOk = [Math]::Abs(($pr5.L - $mr5.R) - $gap3) -le 24 -and [Math]::Abs(($pr5.T - $mr5.T) - ($pr4.T - $mr4.T)) -le 24
Write-Output ("5. FAST fling (500,-200): still snapped = {0} (gap={1} dy={2})" -f $fastOk, ($pr5.L - $mr5.R), ($pr5.T - $mr5.T))

Stop-Process -Id $p.Id -Force
