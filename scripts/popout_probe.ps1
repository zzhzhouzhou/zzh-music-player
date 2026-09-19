# popout_probe v1: real-use pop-out path test.
# launch (no hook) -> drawer hook via ZZH_OPEN_PLAYLIST=1 -> click pop-out button
# -> measure popout window first-response time -> verify it is interactive.
# ASCII only (PS 5.1).
$ErrorActionPreference = 'Stop'
Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public class W {
    public delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr l);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder sb, int max);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll", SetLastError=true)] public static extern IntPtr SendMessageTimeoutW(IntPtr h, uint msg, UIntPtr wp, IntPtr lp, uint flags, uint timeout, out UIntPtr result);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
    public static IntPtr FindByTitle(string want) {
        IntPtr found = IntPtr.Zero;
        EnumWindows((h, l) => {
            var t = new StringBuilder(256); GetWindowTextW(h, t, 256);
            if (t.ToString() == want) { found = h; return false; }
            return true;
        }, IntPtr.Zero);
        return found;
    }
    public static bool Alive(IntPtr h) {
        UIntPtr res;
        IntPtr ok = SendMessageTimeoutW(h, 0, UIntPtr.Zero, IntPtr.Zero, 1, 500, out res);
        return ok != IntPtr.Zero;
    }
}
"@

function Get-Rect([IntPtr]$h) {
    $r = New-Object W+RECT
    [void][W]::GetWindowRect($h, [ref]$r)
    return @{ X = $r.L; Y = $r.T }
}

Get-Process zzhmusicplayer -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 800

$env:ZZH_OPEN_PLAYLIST = "1"   # open the drawer at startup (real-use prerequisite)
if ($args.Count -gt 0 -and $args[0] -eq "empty") {
    # isolate glyph-volume cost: fresh APPDATA => empty playlist, few glyphs
    $env:APPDATA = Join-Path $env:TEMP "zzh-empty-profile"
    New-Item -ItemType Directory -Force -Path $env:APPDATA | Out-Null
    Write-Output "PROFILE=empty"
}
$errLog = Join-Path $PSScriptRoot "probe_err.log"
if (Test-Path $errLog) { Remove-Item $errLog -Force }
$p = Start-Process -FilePath "target\debug\zzhmusicplayer.exe" -PassThru -RedirectStandardError $errLog

# wait for main window to be responsive
$main = [IntPtr]::Zero
foreach ($i in 1..30) {
    Start-Sleep -Milliseconds 500
    $main = [W]::FindByTitle("zzhMusicPlayer")
    if ($main -ne [IntPtr]::Zero -and [W]::Alive($main)) { break }
}
if ($main -eq [IntPtr]::Zero) { Write-Output "FAIL: main window not ready"; Stop-Process -Id $p.Id -Force; exit 1 }
$m = Get-Rect $main
Write-Output ("main ready at {0},{1}" -f $m.X, $m.Y)
Start-Sleep -Milliseconds 600   # let drawer finish its open animation

# click pop-out button: drawer top-right, logical (720-78+9, 16+9) => (651, 25)
$px = $m.X + 651; $py = $m.Y + 25
[void][W]::SetForegroundWindow($main)
Start-Sleep -Milliseconds 200
[void][W]::SetCursorPos($px, $py); Start-Sleep -Milliseconds 120
[W]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 50
[W]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)

# measure: time from click until popout window responds
$sw = [System.Diagnostics.Stopwatch]::StartNew()
$pop = [IntPtr]::Zero; $popAliveAt = -1
while ($sw.ElapsedMilliseconds -lt 12000) {
    if ($pop -eq [IntPtr]::Zero) { $pop = [W]::FindByTitle("zzhMusicPlayer Playlist") }
    if ($pop -ne [IntPtr]::Zero -and $popAliveAt -lt 0 -and [W]::Alive($pop)) {
        $popAliveAt = $sw.ElapsedMilliseconds
        break
    }
    Start-Sleep -Milliseconds 100
}
Write-Output ("POPOUT first-response after click: {0} ms" -f $popAliveAt)
if ($popAliveAt -lt 0) {
    Write-Output "FAIL: popout never responded within 12s"
    Get-Content $errLog -ErrorAction SilentlyContinue
    Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
    exit 1
}

# ---- second cycle: bar playlist button closes popout (toggle), then pop-out again ----
# bar playlist button center: logical (500, 151); pop-out button: (651, 25)
function Click-At([int]$x, [int]$y) {
    [void][W]::SetCursorPos($x, $y); Start-Sleep -Milliseconds 120
    [W]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 50
    [W]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
}
[void][W]::SetForegroundWindow($main); Start-Sleep -Milliseconds 200
# activate-first: some first clicks on an inactive window only activate it
Click-At ($m.X + 360) ($m.Y + 8)
Start-Sleep -Milliseconds 250
Click-At ($m.X + 490) ($m.Y + 151)
# wait until popout is gone (toggle closes it and opens the drawer); time it
$swC = [System.Diagnostics.Stopwatch]::StartNew()
$closed = $false
while ($swC.ElapsedMilliseconds -lt 10000) {
    Start-Sleep -Milliseconds 100
    if ([W]::FindByTitle("zzhMusicPlayer Playlist") -eq [IntPtr]::Zero) { $closed = $true; break }
}
if (-not $closed) {
    Write-Output "FAIL: popout did not close within 10s"
    Get-Content $errLog -ErrorAction SilentlyContinue
    exit 1
}
Write-Output ("popout closed in {0} ms" -f $swC.ElapsedMilliseconds)
Start-Sleep -Milliseconds 400

# toggle again: first click opens the drawer (popout already closed)
Click-At ($m.X + 490) ($m.Y + 151)
Start-Sleep -Milliseconds 800
Click-At ($m.X + 651) ($m.Y + 25)

$sw2 = [System.Diagnostics.Stopwatch]::StartNew()
$pop2 = [IntPtr]::Zero; $pop2AliveAt = -1
while ($sw2.ElapsedMilliseconds -lt 12000) {
    if ($pop2 -eq [IntPtr]::Zero) { $pop2 = [W]::FindByTitle("zzhMusicPlayer Playlist") }
    if ($pop2 -ne [IntPtr]::Zero -and $pop2AliveAt -lt 0 -and [W]::Alive($pop2)) {
        $pop2AliveAt = $sw2.ElapsedMilliseconds
        break
    }
    Start-Sleep -Milliseconds 100
}
Write-Output ("POPOUT second-response after click: {0} ms" -f $pop2AliveAt)
Write-Output "REAL-USE POPOUT OK"
Get-Content $errLog -ErrorAction SilentlyContinue | Select-Object -Last 6
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue

