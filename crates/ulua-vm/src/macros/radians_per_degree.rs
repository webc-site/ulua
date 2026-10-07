use core::f64::consts::PI;

/// 每度对应的弧度值：$\frac{\pi}{180}$（对应 C++ `RADIANS_PER_DEGREE`，基于 [`core::f64::consts::PI`]）。
pub const RADIANS_PER_DEGREE: f64 = PI / 180.0;
