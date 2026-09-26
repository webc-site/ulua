//! Source: `TimeTrace::getClock`（cpp TimeTrace.cpp:68-74）——单调时钟秒值。

use crate::clock_shim::monotonic_seconds;

/// 获取自进程启动以来的单调秒值（f64）。
#[inline]
pub fn get_clock() -> f64 {
  monotonic_seconds()
}
