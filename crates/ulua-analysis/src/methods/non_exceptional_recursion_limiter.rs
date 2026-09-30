//! `non_exceptional_recursion_limiter` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::records::{
  native_stack_guard::NativeStackGuard,
  non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
  recursion_counter::RecursionCounter,
};

impl NonExceptionalRecursionLimiter {
  #[inline]
  pub fn is_ok(&self, limit: i32) -> bool {
    self.native_stack_guard.is_ok() && !(limit > 0 && *self.base.count.get() > limit)
  }
}

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
