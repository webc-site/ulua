use core::sync::atomic::{AtomicI32, Ordering};

/// 镜像 cpp Repl.cpp 全局 globalOptions (单线程 CLI, 原子全局免 unsafe)
static OPTIMIZATION_LEVEL: AtomicI32 = AtomicI32::new(1);
static DEBUG_LEVEL: AtomicI32 = AtomicI32::new(1);

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
