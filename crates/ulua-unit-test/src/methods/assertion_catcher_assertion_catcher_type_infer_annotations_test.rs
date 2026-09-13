use core::{ffi::c_char, sync::atomic::Ordering};

use ulua_common::functions::assert_handler::assert_handler;

use crate::records::assertion_catcher::{ASSERTION_CATCHER_TRIPPED, AssertionCatcher};

unsafe extern "C-unwind" fn assertion_catcher_handler(
  _expression: *const c_char,
  _file: *const c_char,
  _line: i32,
  _function: *const c_char,
) -> i32 {
  ASSERTION_CATCHER_TRIPPED.fetch_add(1, Ordering::SeqCst);
  0
}

impl AssertionCatcher {
  pub fn new() -> Self {
    let oldhook = *assert_handler();
    ASSERTION_CATCHER_TRIPPED.store(0, Ordering::SeqCst);
    *assert_handler() = Some(assertion_catcher_handler);

    Self { oldhook }
  }

  pub fn assertion_catcher(&mut self) {
    self.oldhook = *assert_handler();
    ASSERTION_CATCHER_TRIPPED.store(0, Ordering::SeqCst);
    *assert_handler() = Some(assertion_catcher_handler);
  }

  pub fn tripped() -> i32 {
    ASSERTION_CATCHER_TRIPPED.load(Ordering::SeqCst)
  }
}

impl Default for AssertionCatcher {
  fn default() -> Self {
    Self::new()
  }
}
