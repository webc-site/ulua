use core::sync::atomic::{AtomicI32, Ordering};

static INSTANCE_COUNT: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Counter {
  pub id: i32,
}

impl Counter {
  pub fn new() -> Self {
    let id = INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    Counter { id }
  }

  pub fn reset_instance_count() {
    INSTANCE_COUNT.store(0, Ordering::SeqCst);
  }
}

impl Default for Counter {
  fn default() -> Self {
    Self::new()
  }
}
