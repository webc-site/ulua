use alloc::format;
use std::panic::panic_any;

use crate::records::{
  internal_compiler_error::InternalCompilerError, native_stack_guard::NativeStackGuard,
  recursion_counter::RecursionCounter, recursion_limit_exception::RecursionLimitException,
  recursion_limiter::RecursionLimiter,
};

impl RecursionLimiter {
  /// C++ `RecursionLimiter limiter{system, &count, limit}`
  /// （`Analysis/src/RecursionCounter.cpp:19`：构造即 `++count`，Drop 即 `--count`）。
  ///
  /// 上游计数器模型按 `int*` 传值、跨嵌套限流器共享同一计数（如 TypeInfer
  /// 循环里外层 `_rl` 还活着、递归调用又对同一 `recursionCount` 再建一个），
  /// 所以返回的 RAII 对象不能携带绑定 `count` 的 `&'a mut` 生命周期——否则
  /// 这些合法的上游结构在 Rust 借检查下根本编译不过。[`RecursionCounter`]
  /// 以 `Handle` 别名持有计数器，前置条件与 C++ 传 `int*` 相同：计数器必须
  /// 活得比返回的 `RecursionLimiter` 久（调用点均为函数作用域局部 RAII，
  /// 计数器由外层结构或栈帧持有，由构造保证）。
  pub fn new(system: &str, count: &mut i32, limit: i32) -> RecursionLimiter {
    let mut limiter = RecursionLimiter {
      base: RecursionCounter::recursion_counter_i32(count),
      native_stack_guard: NativeStackGuard { high: 0, low: 0 },
    };

    limiter.native_stack_guard.native_stack_guard();

    if !limiter.native_stack_guard.is_ok() {
      let err = InternalCompilerError::internal_compiler_error_string(format!(
        "Stack overflow in {}",
        system
      ));
      panic_any(err);
    }

    if limit > 0 && *limiter.base.count.get() > limit {
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

    limiter
  }
}
