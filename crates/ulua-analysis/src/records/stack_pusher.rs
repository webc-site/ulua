//! RAII scope-stack 守卫，两个检查器共用：
//! `TypeChecker2.cpp:61-86` 与 `NonStrictTypeChecker.cpp:36-65`（cpp 侧是两份
//! 逐字重复的模板，本仓合并为单一天类型）。

extern crate alloc;

use alloc::vec::Vec;

use crate::records::{
  arena_handle::{Handle, alias, alias_ref},
  scope::Scope,
};

// 构造时入栈，Drop 时断言栈顶并出栈。cpp 用 std::exchange 把被移动的守卫变成
// 空壳；Rust 的移动不会对源对象执行 Drop，因此普通移动语义已经等价。
// #24 b13-tc2-fields：元素句柄化——`Scope` 由 Module arena 独占持有，栈内
// 存 `Handle<Scope>`（类型编码非空），与原 `*mut Scope` 逐位同构（Handle 即
// `NonNull<Scope>`，判空契约见 `arena_handle.rs`）。
#[derive(Debug)]
pub struct StackPusher {
  pub stack: *mut Vec<Handle<Scope>>,
  pub scope: Handle<Scope>,
}

impl StackPusher {
  pub fn new(stack: &mut Vec<Handle<Scope>>, scope: Handle<Scope>) -> Self {
    stack.push(scope);
    Self {
      stack: stack as *mut Vec<Handle<Scope>>,
      scope,
    }
  }
}

impl Drop for StackPusher {
  fn drop(&mut self) {
    if !self.stack.is_null() {
      debug_assert_eq!(alias_ref(self.stack).last().copied(), Some(self.scope));
      alias(self.stack).pop();
    }
  }
}
