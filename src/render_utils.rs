//! 渲染与格式化小工具：兜底背景位图、占位波形、时间文本、背景交叉淡入。

use std::cell::Cell;

use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString};

use crate::TransportState;

/// 尚未完成分析时立即显示的轻量占位波形条数。
pub const WAVE_PLACEHOLDER_BARS: usize = 160;

/// 颜色混合工具。
pub fn mix_rgb(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    [
        (f32::from(a[0]) + (f32::from(b[0]) - f32::from(a[0])) * t).round() as u8,
        (f32::from(a[1]) + (f32::from(b[1]) - f32::from(a[1])) * t).round() as u8,
        (f32::from(a[2]) + (f32::from(b[2]) - f32::from(a[2])) * t).round() as u8,
    ]
}

/// 生成柔和模糊感背景位图：低分辨率纵向渐变 + 若干主题色光斑，
/// 由 UI 平滑放大后呈现“高斯模糊”的柔和观感，体积极小（80×48）。
pub fn render_background(theme: [u8; 3]) -> SharedPixelBuffer<Rgba8Pixel> {
    const W: u32 = 80;
    const H: u32 = 48;
    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(W, H);
    let bytes = buf.make_mut_bytes();
    let stride = W as usize * 4;

    let black = [10u8, 12, 18];
    // 深色覆盖约 70%：颜色整体向深色收敛，仅保留主题色调。
    let top = mix_rgb(theme, black, 0.56);
    let bottom = mix_rgb(theme, black, 0.82);
    // (归一化x, 归一化y, 半径, 光斑色, 强度)：柔和光晕模拟高斯模糊，强度压低。
    let blobs: [(f32, f32, f32, [u8; 3], f32); 4] = [
        (0.24, 0.28, 0.46, mix_rgb(theme, black, 0.42), 0.14),
        (0.72, 0.18, 0.40, mix_rgb(theme, black, 0.50), 0.12),
        (0.52, 0.84, 0.48, mix_rgb(theme, black, 0.32), 0.10),
        (0.92, 0.62, 0.36, mix_rgb(theme, black, 0.46), 0.10),
    ];

    for y in 0..H {
        for x in 0..W {
            let fx = x as f32 / (W - 1) as f32;
            let fy = y as f32 / (H - 1) as f32;
            let mut r = f32::from(top[0]) + (f32::from(bottom[0]) - f32::from(top[0])) * fy;
            let mut g = f32::from(top[1]) + (f32::from(bottom[1]) - f32::from(top[1])) * fy;
            let mut b = f32::from(top[2]) + (f32::from(bottom[2]) - f32::from(top[2])) * fy;
            let a = 0.70 + fy * 0.10; // 暗色覆盖约 70%~80%，毛玻璃轻微透出。
            for &(bx, by, br, col, strength) in &blobs {
                let d = ((fx - bx).powi(2) + (fy - by).powi(2)).sqrt() / br;
                if d < 1.0 {
                    let f = (1.0 - d).powi(2) * strength;
                    r += (f32::from(col[0]) - r) * f;
                    g += (f32::from(col[1]) - g) * f;
                    b += (f32::from(col[2]) - b) * f;
                }
            }
            let i = y as usize * stride + x as usize * 4;
            bytes[i] = r.clamp(0.0, 255.0) as u8;
            bytes[i + 1] = g.clamp(0.0, 255.0) as u8;
            bytes[i + 2] = b.clamp(0.0, 255.0) as u8;
            bytes[i + 3] = (a.clamp(0.0, 1.0) * 255.0) as u8;
        }
    }
    buf
}

/// 把秒数格式化为 m:ss 文本。
pub fn format_time(secs: f32) -> SharedString {
    let total = secs.max(0.0).round() as u64;
    SharedString::from(format!("{}:{:02}", total / 60, total % 60))
}

/// 把背景位图交叉淡入到 UI：新图写入当前隐藏层并翻转可见层，
/// 两层 350ms 透明度动画完成柔和过渡，换曲时背景不再突变。
pub fn push_background(transport: &TransportState, bg: Image, front_showing: &Cell<bool>) {
    if front_showing.get() {
        transport.set_bg_image_back(bg);
        transport.set_bg_front_showing(false);
        front_showing.set(false);
    } else {
        transport.set_bg_image_front(bg);
        transport.set_bg_front_showing(true);
        front_showing.set(true);
    }
}

/// 尚未完成分析时立即显示的轻量占位波形。
pub fn placeholder_bars() -> Vec<f32> {
    (0..WAVE_PLACEHOLDER_BARS)
        .map(|i| {
            let x = i as f32 / WAVE_PLACEHOLDER_BARS as f32;
            0.08 + (x * std::f32::consts::TAU * 3.0).sin().abs() * 0.08
        })
        .collect()
}
