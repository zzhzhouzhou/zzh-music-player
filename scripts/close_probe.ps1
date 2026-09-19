# close_probe.ps1 — isolate the popout close-button path.
# Usage: close_probe.ps1 [exe] [-Drag]
# Opens the popout directly (ZZH_OPEN_PLAYLIST=1), then clicks the panel X.
# With -Drag: performs one native drag on the title strip first.
param(
    [string]$Exe = "target\debug\zzhmusicplayer.exe",
    [switch]$Drag
)
$ErrorActionPreference = "Stop"
$repo = Split-Path $PSScriptRoot
$errLog = Join-Path $repo "scripts\probe_err.log"
Remove-Item $errLog -ErrorAction SilentlyContinue

Get-Process zzhmusicplayer -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 500

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
using System.Text;
public class W {
    public delegate bool EnumProc(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr l);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder sb, int max);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern void mouse_event(uint f, int dx, int dy, uint d, UIntPtr e);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern IntPtr SendMessageTimeoutW(IntPtr h, uint m, UIntPtr wp, IntPtr lp, uint fl, uint to, out UIntPtr res);
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
    public static bool IsAlive(IntPtr h) {
        UIntPtr r; return SendMessageTimeoutW(h, 0x0000, UIntPtr.Zero, IntPtr.Zero, 2, 3000, out r) != IntPtr.Zero;
    }
}
[StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
"@

$env:ZZH_OPEN_POPOUT = "1"
$p = Start-Process $Exe -PassThru -RedirectStandardError $errLog
Start-Sleep -Milliseconds 4500
$pop = [W]::FindByEnum("zzhMusicPlayer Playlist")
if ($pop -eq [IntPtr]::Zero) { Write-Output "NO POPOUT"; Stop-Process -Id $p.Id -Force; exit 1 }
Write-Output "POPOUT alive"

$r0 = New-Object RECT
[void][W]::GetWindowRect($pop, [ref]$r0)
[void][W]::SetForegroundWindow($pop); Start-Sleep -Milliseconds 300

if ($Drag) {
    $bx = $r0.L + 40; $by = $r0.T + 20
    [void][W]::SetCursorPos($bx, $by); Start-Sleep -Milliseconds 100
    [W]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 60
    for ($i = 1; $i -le 6; $i++) { [void][W]::SetCursorPos($bx + 7 * $i, $by + 5 * $i); Start-Sleep -Milliseconds 25 }
    Start-Sleep -Milliseconds 80
    [W]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 500
    if (-not [W]::IsAlive($pop)) { Write-Output "HANG AFTER DRAG"; Stop-Process -Id $p.Id -Force; exit 1 }
    Write-Output "DRAG done"
}

# sweep the header row to find where the X actually sits: click local (x, y)
# candidates one by one, stop when the popout closes.
$hit = $null
foreach ($ly in 25, 18, 31) {
    foreach ($lx in 306, 294, 298, 302, 310, 314, 286, 290, 318) {
        $r1 = New-Object RECT
        [void][W]::GetWindowRect($pop, [ref]$r1)
        [void][W]::SetForegroundWindow($pop); Start-Sleep -Milliseconds 200
        [void][W]::SetCursorPos($r1.L + $lx, $r1.T + $ly); Start-Sleep -Milliseconds 120
        [W]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
        Start-Sleep -Milliseconds 60
        [W]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
        $gone = $false
        for ($i = 0; $i -lt 8; $i++) {
            Start-Sleep -Milliseconds 100
            if ([W]::FindByEnum("zzhMusicPlayer Playlist") -eq [IntPtr]::Zero) { $gone = $true; break }
        }
        Write-Output ("try ({0},{1}) -> {2}" -f $lx, $ly, $(if ($gone) { "CLOSED" } else { "no" }))
        if ($gone) { $hit = "$lx,$ly"; break }
    }
    if ($hit) { break }
}
if ($hit) { Write-Output "X-CLICK OK at $hit (drag=$Drag)" } else { Write-Output "X-CLICK FAILED everywhere (drag=$Drag)" }

Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
exit 0
