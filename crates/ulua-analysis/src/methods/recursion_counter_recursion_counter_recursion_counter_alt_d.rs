use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::recursion_counter::RecursionCounter;

impl RecursionCounter {
  pub fn drop_recursion_counter(&mut self) {
    unsafe {
      LUAU_ASSERT!(*self.count > 0);
      *self.count -= 1;
    }
  }
}

impl Drop for RecursionCounter {
  fn drop(&mut self) {
    self.drop_recursion_counter();
  }
}
