param([string]$Exe = "target\debug\zzhmusicplayer.exe", [int]$WaitSec = 6, [string]$Mode = "popout")
# ASCII-only PS 5.1 script. Mode: popout = start with ZZH_OPEN_POPOUT=1; normal = no hook.
$ErrorActionPreference = "Stop"
Get-Process zzhmusicplayer -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 400

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class L {
    [DllImport("user32.dll", SetLastError=true)] public static extern IntPtr SendMessageTimeoutW(IntPtr h, uint msg, UIntPtr wp, IntPtr lp, uint flags, uint timeout, out UIntPtr result);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr lp);
    public delegate bool EnumProc(IntPtr h, IntPtr lp);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, System.Text.StringBuilder sb, int max);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
    public static IntPtr FindByTitle(string t) {
        IntPtr found = IntPtr.Zero;
        EnumWindows(delegate(IntPtr h, IntPtr lp) {
            if (!IsWindowVisible(h)) return true;
            var sb = new System.Text.StringBuilder(256);
            GetWindowTextW(h, sb, 256);
            if (sb.ToString() == t) { found = h; return false; }
            return true;
        }, IntPtr.Zero);
        return found;
    }
}
"@

if ($Mode -eq "popout") {
    $env:ZZH_OPEN_POPOUT = "1"
} else {
    Remove-Item Env:ZZH_OPEN_POPOUT -ErrorAction SilentlyContinue
}
$p = Start-Process -FilePath $Exe -PassThru -RedirectStandardError "probe_err.log"

# poll responsiveness every 500ms up to $WaitSec seconds; report first-response time
$main = [IntPtr]::Zero; $pop = [IntPtr]::Zero
$mainAt = -1; $popAt = -1
$elapsed = 0
while ($elapsed -lt ($WaitSec * 1000)) {
    Start-Sleep -Milliseconds 500
    $elapsed += 500
    if ($main -eq [IntPtr]::Zero) { $main = [L]::FindByTitle("zzhMusicPlayer") }
    if ($pop -eq [IntPtr]::Zero) { $pop = [L]::FindByTitle("zzhMusicPlayer Playlist") }
    $res = [UIntPtr]::Zero
    if ($main -ne [IntPtr]::Zero -and $mainAt -lt 0) {
        $ok = [L]::SendMessageTimeoutW($main, 0, [UIntPtr]::Zero, [IntPtr]::Zero, 1, 400, [ref]$res)
        if ($ok -ne [IntPtr]::Zero) { $mainAt = $elapsed }
    }
    if ($pop -ne [IntPtr]::Zero -and $popAt -lt 0) {
        $ok2 = [L]::SendMessageTimeoutW($pop, 0, [UIntPtr]::Zero, [IntPtr]::Zero, 1, 400, [ref]$res)
        if ($ok2 -ne [IntPtr]::Zero) { $popAt = $elapsed }
    }
}
Write-Output ("main first-response: {0} ms   pop first-response: {1} ms" -f $mainAt, $popAt)
Write-Output "--- app stderr ---"
Get-Content probe_err.log
Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
