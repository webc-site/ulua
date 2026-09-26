use crate::records::{
  native_stack_guard::NativeStackGuard,
  non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
  recursion_counter::RecursionCounter,
};

impl NonExceptionalRecursionLimiter {
  /// C++ `NonExceptionalRecursionLimiter nerl{&count}`：
  /// 构造即 `++count`，Drop 即 `--count`，不抛异常（超限由 `is_ok` 轮询）。
  pub(crate) fn new(count: &mut i32) -> NonExceptionalRecursionLimiter {
    let base = RecursionCounter::recursion_counter_i32(count);
    let mut limiter = NonExceptionalRecursionLimiter {
      base,
      native_stack_guard: NativeStackGuard { high: 0, low: 0 },
    };
    limiter.native_stack_guard.native_stack_guard();
    limiter
  }
}
