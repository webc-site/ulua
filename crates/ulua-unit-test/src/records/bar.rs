use core::sync::atomic::{AtomicI32, Ordering};

static BAR_COUNT: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Bar {
  pub prop: i32,
}

impl Bar {
  pub fn from_prop(prop: i32) -> Self {
    BAR_COUNT.fetch_add(1, Ordering::SeqCst);
    Self { prop }
  }

  pub fn count() -> i32 {
    BAR_COUNT.load(Ordering::SeqCst)
  }

  pub fn reset_count() {
    BAR_COUNT.store(0, Ordering::SeqCst);
  }
}

impl Clone for Bar {
  fn clone(&self) -> Self {
    Self::from_prop(self.prop)
  }
}

impl Default for Bar {
  fn default() -> Self {
    Self::from_prop(0)
  }
}

impl Drop for Bar {
  fn drop(&mut self) {
    BAR_COUNT.fetch_sub(1, Ordering::SeqCst);
  }
}
