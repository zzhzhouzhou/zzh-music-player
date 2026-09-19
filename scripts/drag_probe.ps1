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
foreach ($round in 1..30) {
    # re-anchor each round: keep the window on screen and the cursor on the title strip
    $cur = Get-Rect $pop
    if ($cur.X -lt 100 -or $cur.Y -lt 0 -or $cur.X -gt 1400) {
        $cur = @{ X = 500; Y = 100 }   # walked offscreen: reset by dragging from wherever it is
    }
    $baseX = [Math]::Min([Math]::Max($cur.X + 40, 60), 1600)
    $baseY = [Math]::Min([Math]::Max($cur.Y + 20, 30), 800)
    $dir = if ($round % 2 -eq 0) { 1 } else { -1 }
    [void][Win]::SetCursorPos($baseX, $baseY); Start-Sleep -Milliseconds 80
    [Win]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)  # LEFTDOWN
    Start-Sleep -Milliseconds 60
    for ($i = 1; $i -le 10; $i++) {
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
}
if ($hangAt -lt 0) { Write-Output "STRESS OK: 30 rounds, UI thread responsive after each" }

$r1 = Get-Rect $pop
Write-Output ("AFTER: {0},{1}" -f $r1.X, $r1.Y)
$dx = $r1.X - $r0.X; $dy = $r1.Y - $r0.Y
if ([Math]::Abs($dx) -gt 10 -and [Math]::Abs($dy) -gt 10) {
    Write-Output "DRAG OK dx=$dx dy=$dy"
} else {
    Write-Output "DRAG FAILED dx=$dx dy=$dy"
}
if (Test-Path $errLog) {
    Write-Output "--- app stderr ---"
    Get-Content $errLog | Select-Object -First 40
}
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
