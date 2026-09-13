use core::ptr::write;

use crate::records::{
  non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
  recursion_counter::RecursionCounter,
};
impl NonExceptionalRecursionLimiter {
  /// # Safety
  /// 调用方须保证 `count` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(
    &mut self,
    count: *mut i32,
  ) {
    unsafe {
      // Initialize the base RecursionCounter with the provided count pointer
      // The base field is the first field, so we can write directly to it
      write(
        &mut self.base as *mut _,
        RecursionCounter::recursion_counter_i32(count),
      );
    }

    self.native_stack_guard.native_stack_guard();
  }
}
