# Drag probe v2: launch -> popout -> SendInput drag -> compare rect. Self-diagnosing.
$ErrorActionPreference = 'Stop'
Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public class Win {
    public delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr l);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder sb, int max);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern IntPtr FindWindowW(string cls, string title);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint flags, int dx, int dy, uint data, UIntPtr extra);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll", SetLastError=true)] public static extern IntPtr SendMessageTimeoutW(IntPtr h, uint msg, UIntPtr wp, IntPtr lp, uint flags, uint timeout, out UIntPtr result);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }

    // liveness probe: returns true if the window's UI thread responds within 3s
    public static bool IsAlive(IntPtr h) {
        UIntPtr res;
        IntPtr ok = SendMessageTimeoutW(h, 0, UIntPtr.Zero, IntPtr.Zero, 1, 3000, out res); // SMTO_BLOCK=1, WM_NULL=0
        return ok != IntPtr.Zero;
    }

    public static string ListWindows(uint target) {
        var sb = new StringBuilder();
        EnumWindows((h, l) => {
            uint pid; GetWindowThreadProcessId(h, out pid);
            if (pid == target) {
                var t = new StringBuilder(256); GetWindowTextW(h, t, 256);
                RECT r; GetWindowRect(h, out r);
                sb.Append(string.Format("  hwnd={0} title='{1}' rect={2},{3} {4}x{5}\n",
                    h, t, r.L, r.T, r.R - r.L, r.B - r.T));
            }
            return true;
        }, IntPtr.Zero);
        return sb.ToString();
    }
    public static IntPtr FindByEnum(string want) {
        IntPtr found = IntPtr.Zero;
        EnumWindows((h, l) => {
            var t = new StringBuilder(256); GetWindowTextW(h, t, 256);
            if (t.ToString() == want) { found = h; return false; }
            return true;
        }, IntPtr.Zero);
        return found;
    }
}
"@

function Get-Rect([IntPtr]$h) {
    $r = New-Object Win+RECT
    [void][Win]::GetWindowRect($h, [ref]$r)
    return @{ X = $r.L; Y = $r.T; W = ($r.R - $r.L); H = ($r.B - $r.T) }
}

function Wait-Window([string]$title, [int]$seconds) {
    $deadline = (Get-Date).AddSeconds($seconds)
    while ((Get-Date) -lt $deadline) {
        $h = [Win]::FindByEnum($title)
        if ($h -ne [IntPtr]::Zero) { return $h }
        Start-Sleep -Milliseconds 250
    }
    return [IntPtr]::Zero
}

Get-Process zzhmusicplayer -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 800

$env:ZZH_OPEN_POPOUT = "1"
$exe = if ($args.Count -gt 0) { $args[0] } else { "target\debug\zzhmusicplayer.exe" }
Write-Output "exe=$exe"
$errLog = Join-Path $PSScriptRoot "probe_err.log"
if (Test-Path $errLog) { Remove-Item $errLog -Force }
$p = Start-Process -FilePath $exe -PassThru -RedirectStandardError $errLog

$pop = Wait-Window "zzhMusicPlayer Playlist" 15
if ($pop -eq [IntPtr]::Zero) {
    Write-Output "FAIL: popout not found after 15s"
    Write-Output "process alive: $(-not $p.HasExited)"
    Write-Output "windows of pid:"
    Write-Output ([Win]::ListWindows([uint32]$p.Id))
    if (-not $p.HasExited) { Stop-Process -Id $p.Id -Force }
    exit 1
}

$r0 = Get-Rect $pop
Write-Output ("BEFORE: {0},{1}" -f $r0.X, $r0.Y)
[void][Win]::SetForegroundWindow($pop)

# wait until the app actually responds before stressing (startup can take seconds)
$alive = $false
foreach ($i in 1..20) {
    if ([Win]::IsAlive($pop)) { $alive = $true; break }
    Start-Sleep -Milliseconds 500
}
if (-not $alive) {
    Write-Output "APP UNRESPONSIVE AT STARTUP (>10s) - aborting stress"
    Get-Content $errLog -ErrorAction SilentlyContinue
    Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
    exit 1
}
Write-Output "app responsive, starting stress"

# stress: repeated native drags on the title strip, liveness check after each round
$hangAt = -1
foreach ($round in 1..20) {
    # anchor on the CURRENT window rect every round (no fake re-anchor:
    # moving the cursor without moving the window makes later clicks miss)
    $cur = Get-Rect $pop
    $baseX = $cur.X + 40; $baseY = $cur.Y + 20
    $dir = if ($round % 2 -eq 0) { 1 } else { -1 }
    [void][Win]::SetCursorPos($baseX, $baseY); Start-Sleep -Milliseconds 80
    [Win]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)  # LEFTDOWN
    Start-Sleep -Milliseconds 60
    for ($i = 1; $i -le 5; $i++) {
        [void][Win]::SetCursorPos($baseX + $dir * [int](6 * $i), $baseY + [int](4 * $i))
        Start-Sleep -Milliseconds 20
    }
    Start-Sleep -Milliseconds 80
    [Win]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)  # LEFTUP
    Start-Sleep -Milliseconds 120
    if (-not [Win]::IsAlive($pop)) {
        Write-Output ("HANG DETECTED at round {0} (UI thread unresponsive >3s)" -f $round)
        $hangAt = $round
        break
    }
    # keep the window on-screen: screen-edge clamping on upward drags biases
    # the drift downward; once the title strip leaves the visible screen the
    # window can no longer be grabbed at all
    $chk = Get-Rect $pop
    if ($chk.Y -gt 550) {
        $up = $chk.Y - 150
        $bx = $chk.X + 40; $by = $chk.Y + 20
        [void][Win]::SetCursorPos($bx, $by); Start-Sleep -Milliseconds 80
        [Win]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
        Start-Sleep -Milliseconds 50
        for ($i = 1; $i -le 8; $i++) {
            [void][Win]::SetCursorPos($bx, $by - [int]($up * $i / 8))
            Start-Sleep -Milliseconds 20
        }
        Start-Sleep -Milliseconds 60
        [Win]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
        Start-Sleep -Milliseconds 150
    }
}
if ($hangAt -lt 0) { Write-Output "STRESS OK: 20 rounds, UI thread responsive after each" }

$r1 = Get-Rect $pop
Write-Output ("AFTER: {0},{1}" -f $r1.X, $r1.Y)
$dx = $r1.X - $r0.X; $dy = $r1.Y - $r0.Y
if ([Math]::Abs($dx) -gt 10 -and [Math]::Abs($dy) -gt 10) {
    Write-Output "DRAG OK dx=$dx dy=$dy"
} else {
    Write-Output "DRAG FAILED dx=$dx dy=$dy"
}

# widened drag zone: full-width top strip (panel grab-ta forwarding), drag from x=250
$c2 = Get-Rect $pop
$wx = $c2.X + 250
$wy = $c2.Y + 10
[void][Win]::SetCursorPos($wx, $wy); Start-Sleep -Milliseconds 100
[Win]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
Start-Sleep -Milliseconds 60
for ($i = 1; $i -le 8; $i++) {
    [void][Win]::SetCursorPos($wx + 8 * $i, $wy + 5 * $i)
    Start-Sleep -Milliseconds 25
}
Start-Sleep -Milliseconds 80
[Win]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
Start-Sleep -Milliseconds 300
$c3 = Get-Rect $pop
$wdx = $c3.X - $c2.X; $wdy = $c3.Y - $c2.Y
if ([Math]::Abs($wdx) -gt 10 -and [Math]::Abs($wdy) -gt 6) {
    Write-Output "WIDE-DRAG OK dx=$wdx dy=$wdy"
} else {
    Write-Output "WIDE-DRAG FAILED dx=$wdx dy=$wdy"
}

# click-after-drag: native drag used to swallow WM_LBUTTONUP and Slint kept the
# pointer grab, making every later click dead (only dragging worked). Verify a
# real click on the close button still works right after a drag.
$c4 = Get-Rect $pop
while ($c4.Y -gt 400) {
    # safety net: drag back toward the top so the close button is on-screen
    $up = $c4.Y - 150
    $bx = $c4.X + 40; $by = $c4.Y + 20
    [void][Win]::SetCursorPos($bx, $by); Start-Sleep -Milliseconds 100
    [Win]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 60
    for ($i = 1; $i -le 10; $i++) {
        [void][Win]::SetCursorPos($bx, $by - [int]($up * $i / 10))
        Start-Sleep -Milliseconds 20
    }
    Start-Sleep -Milliseconds 80
    [Win]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 300
    $c4 = Get-Rect $pop
}
[void][Win]::SetCursorPos($c4.X + 306, $c4.Y + 25); Start-Sleep -Milliseconds 150
[Win]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
Start-Sleep -Milliseconds 60
[Win]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
$gone = $false
for ($i = 0; $i -lt 20; $i++) {
    Start-Sleep -Milliseconds 100
    if ([Win]::FindByEnum("zzhMusicPlayer Playlist") -eq [IntPtr]::Zero) { $gone = $true; break }
}
if ($gone) {
    Write-Output "CLICK-AFTER-DRAG OK (close button responded)"
} else {
    Write-Output "CLICK-AFTER-DRAG FAILED (popout still alive after close click)"
}
if (Test-Path $errLog) {
    Write-Output "--- app stderr ---"
    Get-Content $errLog | Select-Object -First 40
}
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
