//! 平台单调时钟封装（Rust 惯用实现）。
//!
//! 取代 C++ 遗留的跨平台系统调用宏分派（Windows QPC / macOS mach_absolute_time /
//! Linux clock_gettime）。原生平台统一采用 Rust 标准库 [`std::time::Instant`]
//! （内部在 Linux 走 vDSO clock_gettime、macOS 走 mach 时钟、Windows 走 QPC），
//! `wasm32` 目标时钟安全冻结为 0。
//!
//! 100% 内存安全，零外部 OS FFI 依赖（彻底移除 libc、mach2、windows-sys）。

#[cfg(not(target_arch = "wasm32"))]
use std::{sync::OnceLock, time::Instant};

/// 平台基准时钟（首次调用时初始化）。
#[cfg(not(target_arch = "wasm32"))]
static BASE_INSTANT: OnceLock<Instant> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn base_instant() -> Instant {
  *BASE_INSTANT.get_or_init(Instant::now)
}

/// 获取自进程启动以来的高精度单调秒数（f64）。
#[inline]
pub fn monotonic_seconds() -> f64 {
  #[cfg(target_arch = "wasm32")]
  {
    0.0
  }
  #[cfg(not(target_arch = "wasm32"))]
  {
    base_instant().elapsed().as_secs_f64()
  }
}

/// 获取自进程启动以来的单调微秒数（u32），供 TimeTrace 打点使用。
#[inline]
pub(crate) fn monotonic_micros() -> u32 {
  #[cfg(target_arch = "wasm32")]
  {
    0
  }
  #[cfg(not(target_arch = "wasm32"))]
  {
    base_instant().elapsed().as_micros() as u32
  }
}
