use core::sync::atomic::{AtomicBool, Ordering};
#[derive(Debug)]
pub struct FrontendCancellationToken {
  pub cancelled: AtomicBool,
}

// `cancel` / `requested` live in their own method node files
// (methods/frontend_cancellation_token_{cancel,requested}.rs).

impl Clone for FrontendCancellationToken {
  fn clone(&self) -> Self {
    Self {
      cancelled: AtomicBool::new(self.cancelled.load(Ordering::Relaxed)),
    }
  }
}

impl Default for FrontendCancellationToken {
  fn default() -> Self {
    Self {
      cancelled: AtomicBool::new(false),
    }
  }
}
