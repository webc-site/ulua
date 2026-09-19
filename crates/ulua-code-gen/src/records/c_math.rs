unsafe extern "C-unwind" {
  pub fn exp(x: f64) -> f64;
  pub fn pow(x: f64, y: f64) -> f64;
  pub fn fmod(x: f64, y: f64) -> f64;
  pub fn log(x: f64) -> f64;
  pub fn log2(x: f64) -> f64;
  pub fn log10(x: f64) -> f64;
  pub fn ldexp(x: f64, exp: i32) -> f64;
  pub fn round(x: f64) -> f64;
  pub fn frexp(x: f64, exp: *mut i32) -> f64;
  pub fn modf(x: f64, iptr: *mut f64) -> f64;

  pub fn asin(x: f64) -> f64;
  pub fn sin(x: f64) -> f64;
  pub fn sinh(x: f64) -> f64;
  pub fn acos(x: f64) -> f64;
  pub fn cos(x: f64) -> f64;
  pub fn cosh(x: f64) -> f64;
  pub fn atan(x: f64) -> f64;
  pub fn atan2(y: f64, x: f64) -> f64;
  pub fn tan(x: f64) -> f64;
  pub fn tanh(x: f64) -> f64;
}
