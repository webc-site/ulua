use core::sync::atomic::{AtomicI32, Ordering};

pub const CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS: i32 = 0;
pub const CONFORMANCE_INTERRUPT_MODE_INFLOOP: i32 = 1;
pub const CONFORMANCE_INTERRUPT_MODE_TIMEOUT: i32 = 2;

pub struct ConformanceInterruptState {
  pub mode: AtomicI32,
  pub index: AtomicI32,
}

impl ConformanceInterruptState {
  pub const fn new() -> Self {
    Self {
      mode: AtomicI32::new(CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS),
      index: AtomicI32::new(0),
    }
  }

  pub fn reset(&self, mode: i32) {
    self.index.store(0, Ordering::SeqCst);
    self.mode.store(mode, Ordering::SeqCst);
  }

  pub fn index(&self) -> i32 {
    self.index.load(Ordering::SeqCst)
  }
}

impl Default for ConformanceInterruptState {
  fn default() -> Self {
    Self::new()
  }
}

pub static CONFORMANCE_INTERRUPT_STATE: ConformanceInterruptState =
  ConformanceInterruptState::new();
