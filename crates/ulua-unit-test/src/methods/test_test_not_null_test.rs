use core::sync::atomic::Ordering;

use crate::records::test::{TEST_COUNT, Test};
impl Test {
  pub fn new() -> Self {
    TEST_COUNT.fetch_add(1, Ordering::SeqCst);
    Self { x: 0, y: 0.0 }
  }
}

impl Default for Test {
  fn default() -> Self {
    Self::new()
  }
}
