# OCR the captured about-card region with Windows.Media.Ocr (built-in, no deps).
$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Drawing
Add-Type -ReferencedAssemblies System.Runtime.WindowsRuntime -TypeDefinition @"
using System;
using System.Threading.Tasks;
using Windows.Graphics.Imaging;
using Windows.Media.Ocr;
using Windows.Storage.Streams;
public static class OcrRunner {
    public static async Task<string> RunAsync(System.Drawing.Bitmap bmp) {
        var engine = OcrEngine.TryCreateFromUserProfileLanguages();
        if (engine == null) return "NO-OCR-ENGINE";
        var ms = new InMemoryRandomAccessStream();
        bmp.Save(ms.AsStreamForWrite(), System.Drawing.Imaging.ImageFormat.Png);
        ms.Seek(0);
        var decoder = await BitmapDecoder.CreateAsync(ms);
        var soft = await decoder.GetSoftwareBitmapAsync();
        var result = await engine.RecognizeAsync(soft);
        return result.Text;
    }
}
"@ -ErrorAction SilentlyContinue

$bmp = [System.Drawing.Bitmap]::FromFile("E:\zzh-music-player\scripts\about_capture.png")
$task = [OcrRunner]::RunAsync($bmp)
$task.Wait(15000) | Out-Null
if ($task.IsCompleted) { $task.Result } else { "OCR-TIMEOUT" }
