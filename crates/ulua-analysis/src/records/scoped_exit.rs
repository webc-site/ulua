use alloc::boxed::Box;

use ulua_common::LUAU_ASSERT;

#[derive(Default)]
pub struct ScopedExit {
  pub(crate) func: Option<Box<dyn FnOnce()>>,
}

impl ScopedExit {
  pub fn new(f: Box<dyn FnOnce()>) -> Self {
    LUAU_ASSERT!(true); // In C++, LUAU_ASSERT(func) checks if the std::function is non-null.
    Self { func: Some(f) }
  }
}

impl Drop for ScopedExit {
  fn drop(&mut self) {
    if let Some(f) = self.func.take() {
      f();
    }
  }
}

// ScopedExit in C++ is used for RAII-style cleanup.
// The C++ implementation uses std::function<void()>, which is roughly Box<dyn FnOnce()>.
// We use Option to allow moving the function out during drop or swap (move semantics).

impl ScopedExit {}

unsafe impl Send for ScopedExit {}
unsafe impl Sync for ScopedExit {}
