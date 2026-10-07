use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{arena_handle::Handle, recursion_counter::RecursionCounter};

impl RecursionCounter {
  /// C++ `RecursionCounter::RecursionCounter(int* count) : count(count) { ++(*count); }`
  /// (`Analysis/src/RecursionCounter.cpp:14`)。
  ///
  /// 借用契约（安全函数体，无 `unsafe`）：`count` 为宿主结构体的
  /// `i32` 字段独占借用；守卫内部只保存 [`Handle`] 别名，构造语句结束后
  /// `&mut` 借用即释放，之后宿主仍可照常经 `&mut self` 递归调用——与 C++
  /// 持有 `int*` 的前置条件一致：计数器必须活得比返回的守卫久（调用点均为
  /// 函数作用域局部 RAII，计数器由外层结构或栈帧持有，由构造保证）。
  #[inline]
  pub fn recursion_counter_i32(count: &mut i32) -> RecursionCounter {
    let count = Handle::from_mut(count);
    *count.get_mut() += 1;
    RecursionCounter { count }
  }

  pub fn drop_recursion_counter(&mut self) {
    let count = self.count.get();
    LUAU_ASSERT!(*count > 0);
    *self.count.get_mut() -= 1;
  }
}

impl Drop for RecursionCounter {
  fn drop(&mut self) {
    self.drop_recursion_counter();
  }
}
