use ulua_common::functions::assert_handler::assert_handler;

use crate::records::assertion_catcher::AssertionCatcher;

impl Drop for AssertionCatcher {
  fn drop(&mut self) {
    *assert_handler() = self.oldhook;
  }
}
