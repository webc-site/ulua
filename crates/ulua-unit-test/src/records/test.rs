use core::sync::atomic::{AtomicI32, Ordering};

pub static TEST_COUNT: AtomicI32 = AtomicI32::new(0);

#[derive(Debug)]
pub struct Test {
  pub x: i32,
  pub y: f32,
}

impl Test {
  pub fn count() -> i32 {
    TEST_COUNT.load(Ordering::SeqCst)
  }
}

impl Drop for Test {
  fn drop(&mut self) {
    TEST_COUNT.fetch_sub(1, Ordering::SeqCst);
  }
}
