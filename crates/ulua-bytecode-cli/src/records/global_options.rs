use core::sync::atomic::{AtomicI32, Ordering};

/// cpp 默认优化级别 (`globalOptions.optimizationLevel = 1`)
pub const DEFAULT_OPTIMIZATION_LEVEL: i32 = 1;
/// cpp 默认调试级别 (`globalOptions.debugLevel = 1`)
pub const DEFAULT_DEBUG_LEVEL: i32 = 1;

/// 镜像 cpp 全局 globalOptions: 单线程 CLI 专用原子全局
static OPTIMIZATION_LEVEL: AtomicI32 = AtomicI32::new(DEFAULT_OPTIMIZATION_LEVEL);
static DEBUG_LEVEL: AtomicI32 = AtomicI32::new(DEFAULT_DEBUG_LEVEL);

#[inline]
pub fn get_optimization_level() -> i32 {
  OPTIMIZATION_LEVEL.load(Ordering::Relaxed)
}

#[inline]
pub fn set_optimization_level(level: i32) {
  OPTIMIZATION_LEVEL.store(level, Ordering::Relaxed);
}

#[inline]
pub fn get_debug_level() -> i32 {
  DEBUG_LEVEL.load(Ordering::Relaxed)
}

#[inline]
pub fn set_debug_level(level: i32) {
  DEBUG_LEVEL.store(level, Ordering::Relaxed);
}
