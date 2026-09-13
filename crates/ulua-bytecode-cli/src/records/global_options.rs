use core::{
  ffi::c_int,
  sync::atomic::{AtomicI32, Ordering},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalOptions {
  pub optimization_level: c_int,
  pub debug_level: c_int,
}

impl Default for GlobalOptions {
  fn default() -> Self {
    Self {
      optimization_level: 1,
      debug_level: 1,
    }
  }
}

static OPTIMIZATION_LEVEL: AtomicI32 = AtomicI32::new(1);
static DEBUG_LEVEL: AtomicI32 = AtomicI32::new(1);

#[inline]
pub fn get_optimization_level() -> c_int {
  OPTIMIZATION_LEVEL.load(Ordering::Relaxed)
}

#[inline]
pub fn set_optimization_level(level: c_int) {
  OPTIMIZATION_LEVEL.store(level, Ordering::Relaxed);
}

#[inline]
pub fn get_debug_level() -> c_int {
  DEBUG_LEVEL.load(Ordering::Relaxed)
}

#[inline]
pub fn set_debug_level(level: c_int) {
  DEBUG_LEVEL.store(level, Ordering::Relaxed);
}
