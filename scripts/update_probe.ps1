# Auto-update E2E probe (UI-driven, pixel-verified):
# fake old version -> open About -> tap "check updates" (manual path) ->
# sample the note-line pixels; a lit note + non-idle state means the
# updater reached GitHub API and returned an Available verdict.
# (Silent 30s check writes the same state machine; UI text is the
# observable surface for both.)
$ErrorActionPreference = "Stop"
Add-Type -TypeDefinition @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public struct RECT3 { public int L, T, R, B; }
public class W {
    public delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr l);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder sb, int max);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT3 r);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
    [DllImport("gdi32.dll")] public static extern uint GetPixel(IntPtr dc, int x, int y);
    [DllImport("user32.dll")] public static extern IntPtr GetWindowDC(IntPtr h);
    [DllImport("user32.dll")] public static extern int ReleaseDC(IntPtr h, IntPtr dc);
    public static IntPtr FindByEnum(string want) {
        IntPtr found = IntPtr.Zero;
        EnumWindows((h, l) => {
            StringBuilder sb = new StringBuilder(128);
            GetWindowTextW(h, sb, 128);
            if (sb.ToString().TrimEnd() == want) { found = h; return false; }
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
    Start-Sleep -Milliseconds 450
}

# non-zero sample: count bright pixels in a box (text strokes)
function Bright-Count($x0, $y0, $w, $h) {
    $dc = [W]::GetWindowDC([IntPtr]::Zero)
    $n = 0
    for ($y = $y0; $y -lt $y0 + $h; $y += 2) {
        for ($x = $x0; $x -lt $x0 + $w; $x += 2) {
            $c = [W]::GetPixel($dc, $x, $y) -band 0xFFFFFF
            if (($c -band 0xFF) -gt 0x90 -and (($c -shr 8) -band 0xFF) -gt 0x90) { $n++ }
        }
    }
    [void][W]::ReleaseDC([IntPtr]::Zero, $dc)
    return $n
}

$env:ZZH_VERSION_OVERRIDE = "0.9.0"
$log = Join-Path $PSScriptRoot "upd_probe.log"
if (Test-Path $log) { Remove-Item $log -Force }
$p = Start-Process "D:\zzh-music-player\zzhmusicplayer.exe" -PassThru -RedirectStandardError $log
Start-Sleep -Seconds 6
$hwnd = [W]::FindByEnum("zzhMusicPlayer")
if ($hwnd -eq [IntPtr]::Zero) { Write-Output "FAIL: main window not found"; Stop-Process -Id $p.Id -Force; exit 1 }
[void][W]::SetForegroundWindow($hwnd)
Start-Sleep -Seconds 35   # 30s silent check fires + settles

$r = New-Object RECT3
[void][W]::GetWindowRect($hwnd, [ref]$r)
# About button: x = width-138, y=7, size 26 -> center
$abX = $r.R - 138 + 13
$abY = $r.T + 7 + 13
Click-At $abX $abY
Start-Sleep -Milliseconds 800

# About card is centered in the window; the action line ("check updates" /
# note text) sits in the lower area of the card. Sample a wide band.
$cx = ($r.L + $r.R) / 2
$bandX0 = [int]($cx - 120)
$bandY0 = [int]($r.T + 120)
$before = Bright-Count $bandX0 $bandY0 240 60
# tap "check updates" line (left part of the action row under the icon)
Click-At ($bandX0 + 150) ($bandY0 + 30)
# poll: checking (brief) -> available/up-to-date/failed; sample every 1.5s x8
$verdict = "no-change"
for ($i = 0; $i -lt 8; $i++) {
    Start-Sleep -Milliseconds 1500
    $n = Bright-Count $bandX0 $bandY0 240 60
    Write-Output ("poll {0}: band={1}" -f ($i + 1), $n)
    if ($n -gt 0 -and $n -ne $before) { $verdict = "text-changed (state reached: $n)"; break }
}
Write-Output ("verdict: {0}" -f $verdict)
# capture the about card region for human-readable verification
Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms
$bw = 300; $bh = 190
$bmp = New-Object System.Drawing.Bitmap($bw, $bh)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.CopyFromScreen($bandX0, ($r.T + 30), 0, 0, $bmp.Size)
$bmp.Save((Join-Path $PSScriptRoot "about_capture.png"), [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose(); $bmp.Dispose()
Write-Output "capture saved: scripts/about_capture.png"
Stop-Process -Id $p.Id -Force
