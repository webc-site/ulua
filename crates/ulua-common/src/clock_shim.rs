//! 平台时钟 FFI 统一封装。
//!
//! C++ 的平台时钟代码有两处完全重复：`VM/src/lperf.cpp` 的
//! `clock_timestamp`/`clock_period`（供 `lua_clock`，即 `os.clock`）与
//! `Common/src/TimeTrace.cpp` 的 `getClockTimestamp`/`getClockPeriod`
//! （供 TimeTrace 计时）。Rust 移植后两组实现的**平台分派语义**存在刻意
//! 差异（vm 侧含 emscripten 分支，对齐 `lperf.cpp`；common 侧为无 libc 的
//! `wasm32-unknown-unknown` 冻结时钟），故不做函数级合并，仅把平台 FFI
//! 收拢到本模块，供本 crate 与 `ulua-vm` 的上层实现共用：
//!
//! - Windows：`windows-sys` 的 QPC
//! - macOS：`mach2`（取代手写 `extern "C"` 声明）
//! - Linux/FreeBSD：`libc`（取代手写 `extern "C"` 声明）
//! - 兜底 `clock()`：保持裸符号声明——`wasm32-unknown-unknown` 上由
//!   [`crate::wasm_libc`] 的垫片提供符号，native 由平台 libc 提供。

use core::ffi::c_long;

/// 兜底时钟周期（秒）：POSIX XSI 规定 `CLOCKS_PER_SEC` 为 1e6
/// （glibc/Darwin/musl 一致），编译期常量替代 C++ 的
/// `1.0 / double(CLOCKS_PER_SEC)`，避免 extern static 链接。
pub const CLOCK_TICK_PERIOD: f64 = 1e-6;

/// Windows QPC 计数（`QueryPerformanceCounter`），对应 C++ `clock_timestamp`
/// 的 `_WIN32` 分支。
#[cfg(target_os = "windows")]
pub fn qpc_ticks() -> f64 {
  use windows_sys::Win32::System::Performance::QueryPerformanceCounter;

  let mut ticks: i64 = 0;
  // SAFETY: 输出指针指向合法的本地 i64，QPC 无其它前置条件。
  unsafe { QueryPerformanceCounter(&mut ticks) };
  ticks as f64
}

/// Windows QPC 周期（秒/计数），即 `1 / QueryPerformanceFrequency`。
#[cfg(target_os = "windows")]
pub fn qpc_period() -> f64 {
  use windows_sys::Win32::System::Performance::QueryPerformanceFrequency;

  let mut freq: i64 = 0;
  // SAFETY: 输出指针指向合法的本地 i64，QPF 无其它前置条件。
  unsafe { QueryPerformanceFrequency(&mut freq) };
  1.0 / (freq as f64)
}

/// macOS `mach_absolute_time` 原始计数，单位由 timebase 决定（见 [`mach_period`]）。
#[cfg(target_os = "macos")]
pub fn mach_ticks() -> f64 {
  use mach2::mach_time::mach_absolute_time;

  // SAFETY: 无参数、无前置条件的系统调用。
  unsafe { mach_absolute_time() as f64 }
}

/// macOS timebase 周期（秒/计数）：`numer / denom * 1e-9`。
#[cfg(target_os = "macos")]
pub fn mach_period() -> f64 {
  use mach2::mach_time::{mach_timebase_info, mach_timebase_info_data_t};

  let mut info = mach_timebase_info_data_t::default();
  // SAFETY: 输出指针指向合法的本地结构体，系统调用只写入它。
  unsafe { mach_timebase_info(&mut info) };
  f64::from(info.numer) / f64::from(info.denom) * 1e-9
}

/// `CLOCK_MONOTONIC` 当前值（纳秒），对应 C++ `clock_timestamp` 的
/// `__linux__`/`__FreeBSD__` 分支。
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn monotonic_ns() -> f64 {
  let mut now = libc::timespec {
    tv_sec: 0,
    tv_nsec: 0,
  };
  // SAFETY: 输出指针指向合法的本地 timespec；CLOCK_MONOTONIC 为常量 clk_id。
  unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now) };
  (now.tv_sec as f64) * 1e9 + now.tv_nsec as f64
}

/// emscripten 毫秒时钟（`emscripten_get_now`），对应 C++ `clock_timestamp`
/// 的 `__EMSCRIPTEN__` 分支。
#[cfg(target_os = "emscripten")]
pub fn emscripten_now() -> f64 {
  unsafe extern "C" {
    fn emscripten_get_now() -> f64;
  }
  // SAFETY: 无参数、无前置条件的宿主函数。
  unsafe { emscripten_get_now() }
}

/// 兜底 `clock()` 处理器时间（`CLOCKS_PER_SEC` 单位）。裸符号声明，
/// 无 crate 依赖：native 链接平台 libc，`wasm32-unknown-unknown` 链接
/// [`crate::wasm_libc`] 的垫片（恒返回 0，时钟冻结）。
pub fn clock_ticks() -> f64 {
  unsafe extern "C" {
    fn clock() -> c_long;
  }
  // SAFETY: 无参数、无前置条件的 libc 函数。
  unsafe { clock() as f64 }
}
