// wasm 分支（冻结周期 1e-9）不走 clock_shim，门控避免 unused import。
#[cfg(not(target_arch = "wasm32"))]
use crate::clock_shim;

/// 对应 C++ `TimeTrace.cpp` 的 `getClockPeriod`：TimeTrace 时钟单个 tick 的
/// 秒数。与 `get_clock_timestamp` 配对；平台 FFI 统一收拢在
/// [`crate::clock_shim`]（与 vm 侧 `clock_period` 共享，仅分派策略不同）。
pub(crate) fn get_clock_period() -> f64 {
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
  #[cfg(target_arch = "wasm32")]
  {
    // `wasm32-unknown-unknown` exposes no clock source (no libc, no WASI),
    // so TimeTrace profiling is a no-op. A 1 ns period keeps the
    // `period * timestamp` math finite alongside the frozen timestamp.
    1e-9
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
    // C++ 兜底 `1.0 / double(CLOCKS_PER_SEC)`；POSIX XSI 规定其值为 1e6
    //（glibc/Darwin/musl 一致）。libc crate 在本 crate 是 linux/freebsd
    // 门控依赖，此分支恰在其它目标编译，故用编译期常量。
    clock_shim::CLOCK_TICK_PERIOD
  }
}
