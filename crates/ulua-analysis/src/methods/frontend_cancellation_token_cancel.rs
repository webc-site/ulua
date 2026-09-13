use core::sync::atomic::Ordering;

use crate::records::frontend_cancellation_token::FrontendCancellationToken;

impl FrontendCancellationToken {
  pub fn cancel(&self) {
    self.cancelled.store(true, Ordering::Relaxed);
  }
}
