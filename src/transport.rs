//! 传输域的纯 Rust 状态：跳转等待（seek 回执过滤）与主题色补间。
//! 不放进 Slint 全局：它们带方法逻辑与内部可变状态，由 app 组合根持有并共享。

use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant};

use crate::TransportState;
use crate::waveform_generator;

/// 跳转确认超时：异常设备没有回执时也不会永久锁住进度。
pub const SEEK_CONFIRM_TIMEOUT: Duration = Duration::from_millis(1500);
/// 收到 seek 回执后再屏蔽一小段时间，吸收播放线程中已经排队的旧位置事件。
pub const SEEK_SETTLE_WINDOW: Duration = Duration::from_millis(180);

/// 跳转状态：等待引擎确认，或确认后的短暂稳定窗口。
/// lock 为真表示显示钉在目标上（点击/拖拽跳转）；为 false 时（方向键快进快退）
/// 显示走 played-frac 的 200ms 插值动画平滑滑向目标，仅过滤陈旧位置事件。
#[derive(Clone, Copy)]
pub enum SeekState {
    Pending {
        target: f32,
        since: Instant,
        lock: bool,
    },
    Settling {
        target: f32,
        until: Instant,
        lock: bool,
    },
}

/// 主题色补间状态：记录目标色与进行中的过渡（起点 HSL, 目标 HSL, 开始时刻）。
/// Slint 全局组件不支持属性动画，由 33ms 粒子计时器驱动逐步推进，
/// 让波形高亮 / 按钮 / 控制胶囊叠色随换曲平滑过渡。
/// 插值在 HSL 空间沿色相环最短弧进行：RGB 插值跨色相过渡会中途发灰
/// （先变暗再变亮），色相插值则直接经过相邻色相（绿→青→蓝→紫）。
pub type HslColor = (f32, f32, f32);
pub type ActiveThemeTween = (HslColor, HslColor, Instant);

#[derive(Default)]
pub struct ThemeTween {
    target: Cell<[u8; 3]>,
    active: RefCell<Option<ActiveThemeTween>>,
}

impl ThemeTween {
    /// 以初始主题色构造（应与 main.slint 的默认 theme-color 一致）。
    pub fn new(initial: [u8; 3]) -> Self {
        Self {
            target: Cell::new(initial),
            ..Default::default()
        }
    }

    /// 启动到目标色的过渡；颜色相同则直接落定。约 400ms，ease-out。
    pub fn start(&self, to: [u8; 3]) {
        let from = self.target.get();
        if from == to {
            self.active.borrow_mut().take();
            return;
        }
        self.target.set(to);
        let from_hsl = waveform_generator::rgb_to_hsl(from);
        let to_hsl = waveform_generator::rgb_to_hsl(to);
        *self.active.borrow_mut() = Some((from_hsl, to_hsl, Instant::now()));
    }

    /// 由周期计时器每 tick 调用：推进过渡并返回是否仍需继续。
    pub fn tick(&self, transport: &TransportState, dt_step: f32) -> bool {
        let Some(((fh, fs, fl), (th, ts, tl), started)) = self.active.borrow().as_ref().copied()
        else {
            return false;
        };
        let t = (started.elapsed().as_secs_f32() / (dt_step * 12.0)).min(1.0);
        let k = 1.0 - (1.0 - t) * (1.0 - t); // ease-out
        // 色相沿环最短弧过渡（-180 ~ +180），饱和度 / 亮度线性插值。
        let dh = ((th - fh + 540.0).rem_euclid(360.0)) - 180.0;
        let [r, g, b] =
            waveform_generator::hsl_to_rgb(fh + dh * k, fs + (ts - fs) * k, fl + (tl - fl) * k);
        transport.set_theme_color(slint::Color::from_rgb_u8(r, g, b));
        if t >= 1.0 {
            self.active.borrow_mut().take();
            false
        } else {
            true
        }
    }
}
