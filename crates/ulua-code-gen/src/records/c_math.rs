//! libm 宿主数学绑定：正当的宿主 FFI（类比 `ulua-vm` 的宿主 `fwrite`），
//! 不属于待清理的 wasm_libc shim 类 FFI 残留。

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
