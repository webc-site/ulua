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
      // 有意保留 `String` 载荷（§6 例外的另一面，勿「顺手」改成 `panic_any`）：
      // 消费侧就是按文本匹配的——`reduce_type_functions_type_function.rs` 的
      // `is_recursion_limit_panic` 以前缀 "Internal recursion counter limit
      // exceeded" 判定 C++ `catch (RecursionLimitException&)`，`lua_d_rawrunprotected`
      // 也把 String 载荷折成 Lua 错误消息；换成类型载荷会让这两处 catch 全部失配。
      // 早期版本改抛 `err.base.what()` 的 C 串，把 String 字节当 NUL 结尾串读，
      // 越界带出尾部垃圾字符（如 "...in areEqual5\u{fffd}"），故此处只抛自有 String。
      panic!("{}", err.base.message);
    }

    limiter
  }
}
