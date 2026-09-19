use ulua_common::functions::assert_handler::set_assert_handler;

use crate::records::assertion_catcher::AssertionCatcher;

impl Drop for AssertionCatcher {
  fn drop(&mut self) {
    set_assert_handler(self.oldhook);
  }
}
