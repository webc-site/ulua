use ulua_common::clock_shim;

/// 对应 C++ `lperf.cpp` 的 `clock_timestamp`：平台高精度单调时钟的原始
/// 计数（乘以 [`super::clock_period::clock_period`] 得秒）。FFI 统一收拢在
/// `ulua_common::clock_shim`。
pub(crate) fn clock_timestamp() -> f64 {
  #[cfg(target_os = "windows")]
  {
    clock_shim::qpc_ticks()
  }
  #[cfg(target_os = "macos")]
  {
    clock_shim::mach_ticks()
  }
  #[cfg(any(target_os = "linux", target_os = "freebsd"))]
  {
    clock_shim::monotonic_ns()
  }
  #[cfg(target_os = "emscripten")]
  {
    clock_shim::emscripten_now()
  }
  #[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "freebsd",
    target_os = "emscripten"
  )))]
  {
    clock_shim::clock_ticks()
  }
}
