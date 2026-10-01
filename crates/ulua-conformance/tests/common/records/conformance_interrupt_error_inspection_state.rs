use core::sync::atomic::{AtomicI32, Ordering};

pub struct ConformanceInterruptErrorInspectionState {
  pub target: AtomicI32,
  pub step: AtomicI32,
}

impl ConformanceInterruptErrorInspectionState {
  pub const fn new() -> Self {
    Self {
      target: AtomicI32::new(0),
      step: AtomicI32::new(0),
    }
  }

  pub fn reset(&self, target: i32) {
    self.target.store(target, Ordering::SeqCst);
    self.step.store(0, Ordering::SeqCst);
  }
}

impl Default for ConformanceInterruptErrorInspectionState {
  fn default() -> Self {
    Self::new()
  }
}

pub static CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE: ConformanceInterruptErrorInspectionState =
  ConformanceInterruptErrorInspectionState::new();
