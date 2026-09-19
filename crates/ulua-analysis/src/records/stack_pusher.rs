//! RAII scope-stack 守卫，两个检查器共用：
//! `TypeChecker2.cpp:61-86` 与 `NonStrictTypeChecker.cpp:36-65`（cpp 侧是两份
//! 逐字重复的模板，本仓合并为单一天类型）。

extern crate alloc;

use alloc::vec::Vec;

use crate::records::scope::Scope;

// 构造时入栈，Drop 时断言栈顶并出栈。cpp 用 std::exchange 把被移动的守卫变成
// 空壳；Rust 的移动不会对源对象执行 Drop，因此普通移动语义已经等价。
#[derive(Debug)]
pub struct StackPusher {
  pub stack: *mut Vec<*mut Scope>,
  pub scope: *mut Scope,
}

impl StackPusher {
  /// # Safety
  /// `stack`、`scope` 必须在守卫的整个生命周期内有效，且 `stack` 不被其他
  /// 借用同时修改（cpp 侧同一检查器实例内的独占访问）。
  pub unsafe fn new(stack: *mut Vec<*mut Scope>, scope: *mut Scope) -> Self {
    unsafe {
      (*stack).push(scope);
    }
    Self { stack, scope }
  }
}

impl Drop for StackPusher {
  fn drop(&mut self) {
    if !self.stack.is_null() {
      unsafe {
        debug_assert_eq!((*self.stack).last().copied(), Some(self.scope));
        (*self.stack).pop();
      }
    }
  }
}
