use alloc::format;
use core::ptr::write;
use std::panic::panic_any;

use crate::records::{
  internal_compiler_error::InternalCompilerError, recursion_counter::RecursionCounter,
  recursion_limit_exception::RecursionLimitException, recursion_limiter::RecursionLimiter,
};
impl RecursionLimiter {
  /// # Safety
  /// 调用方须保证 `count` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn recursion_limiter_recursion_limiter(
    &mut self,
    system: &str,
    count: *mut i32,
    limit: i32,
  ) {
    unsafe {
      write(
        &mut self.base as *mut RecursionCounter,
        RecursionCounter::recursion_counter_i32(count),
      );
    }

    self.native_stack_guard.native_stack_guard();

    if !self.native_stack_guard.is_ok() {
      let err = InternalCompilerError::internal_compiler_error_string(format!(
        "Stack overflow in {}",
        system
      ));
      panic_any(err);
    }

    if limit > 0 && unsafe { *self.base.count > limit } {
      let err = RecursionLimitException::new(system);
      // Panic with the exception's owned message String. The previous
      // `CStr::from_ptr(err.base.what())` read the String's bytes as a
      // NUL-terminated C string and over-ran past the end, so the surfaced
      // message carried trailing garbage (e.g. "...in areEqual5\u{fffd}").
      // lua_d_rawrunprotected downcasts this String payload into a LuaErrrun
      // message, so a user type function that trips the limit reports
      // "'<fn>' type function errored at runtime: Internal recursion counter
      // limit exceeded in <system>" (type_function_user_udtf_areequal_*).
      panic!("{}", err.base.message);
    }
  }
}
