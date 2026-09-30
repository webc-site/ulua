use core::sync::atomic::{AtomicBool, AtomicI32, Ordering};

pub struct ConformanceTagMethodErrorState {
  pub index: AtomicI32,
  pub lua_break: AtomicBool,
}

impl ConformanceTagMethodErrorState {
  pub const fn new() -> Self {
    Self {
      index: AtomicI32::new(0),
      lua_break: AtomicBool::new(false),
    }
  }

  pub fn reset(&self, lua_break: bool) {
    self.index.store(0, Ordering::SeqCst);
    self.lua_break.store(lua_break, Ordering::SeqCst);
  }
}

impl Default for ConformanceTagMethodErrorState {
  fn default() -> Self {
    Self::new()
  }
}

pub static CONFORMANCE_TAG_METHOD_ERROR_STATE: ConformanceTagMethodErrorState =
  ConformanceTagMethodErrorState::new();
