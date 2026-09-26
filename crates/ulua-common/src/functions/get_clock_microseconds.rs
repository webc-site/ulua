use crate::clock_shim::monotonic_micros;

/// cpp `TimeTrace::getClockMicroseconds` 镜像工件（sync-cpp 维护）；全仓仅本
/// crate 打点机制消费（秒值版 `get_clock` 才是跨 crate 公共面），降 `pub(crate)`。
#[inline]
pub(crate) fn get_clock_microseconds() -> u32 {
  monotonic_micros()
}
