// wasm 分支（时钟冻结为 0）不走 clock_shim，门控避免 unused import。
#[cfg(not(target_arch = "wasm32"))]
use crate::clock_shim;

/// 对应 C++ `TimeTrace.cpp` 的 `getClockTimestamp`：TimeTrace 用的平台
/// 时钟原始计数。与 vm 侧 `clock_timestamp`（`lperf.cpp`）是 C++ 里两处
/// 重复代码的移植，但 wasm 语义刻意不同（emscripten 也冻结为 0），故
/// 只共享 FFI（[`crate::clock_shim`]），不合并函数。
pub(crate) fn get_clock_timestamp() -> f64 {
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
  #[cfg(target_arch = "wasm32")]
  {
    // `wasm32-unknown-unknown` has no clock source; the profiler clock is
    // frozen at 0. See `get_clock_period` for the matching no-op period.
    0.0
  }
  #[cfg(all(
    not(target_arch = "wasm32"),
    not(any(
      target_os = "windows",
      target_os = "macos",
      target_os = "linux",
      target_os = "freebsd"
    ))
  ))]
  {
    // 裸符号 clock()（clock_shim），不依赖 libc crate——其在本 crate 是
    // linux/freebsd 门控依赖，此兜底分支恰在其它目标上编译。
    clock_shim::clock_ticks()
  }
}
