use ulua_common::clock_shim;

/// 对应 C++ `lperf.cpp` 的 `clock_period`：单个时钟 tick 的秒数。
/// FFI 统一收拢在 `ulua_common::clock_shim`。
pub(crate) fn clock_period() -> f64 {
  #[cfg(target_os = "windows")]
  {
    clock_shim::qpc_period()
  }
  #[cfg(target_os = "macos")]
  {
    clock_shim::mach_period()
  }
  #[cfg(any(target_os = "linux", target_os = "freebsd"))]
  {
    1e-9
  }
  // 仅 emscripten 的计数单位是毫秒（C++ `__EMSCRIPTEN__` 分支）。此处原为
  // `target_arch = "wasm32"`，误将 1e-3 扩到所有 wasm：`wasm32-wasip1` 上
  // timestamp 走 `clock()` 兜底（秒），周期须为 1e-6 才匹配。
  #[cfg(target_os = "emscripten")]
  {
    1e-3
  }
  #[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "freebsd",
    target_os = "emscripten"
  )))]
  {
    clock_shim::CLOCK_TICK_PERIOD
  }
}
