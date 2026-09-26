//! RAII scope-stack 守卫，两个检查器共用：
//! `TypeChecker2.cpp:61-86` 与 `NonStrictTypeChecker.cpp:36-65`（cpp 侧是两份
//! 逐字重复的模板，本仓合并为单一天类型）。

extern crate alloc;

use alloc::vec::Vec;

use crate::records::{arena_handle::Handle, scope::Scope};

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
  /// # Safety
  /// `stack`、`scope` 必须在守卫的整个生命周期内有效，且 `stack` 不被其他
  /// 借用同时修改（cpp 侧同一检查器实例内的独占访问）。`scope` 的句柄目标
  /// 须满足 `Handle` 的类型级契约（存活至守卫结束）。
  pub unsafe fn new(stack: *mut Vec<Handle<Scope>>, scope: Handle<Scope>) -> Self {
    // Safety: 构造契约要求 stack 指向守卫持有者（检查器）自有的 scope 栈
    // Vec，且在守卫存活期内不被其它借用触碰——cpp 侧同一检查器实例内串行
    // 进出作用域，push 只在守卫创建点发生一次；(*stack) 重建瞬态 &mut 于
    // push 语句结束即失效，单线程下无并存别名。scope 仅作为 ptr 值入栈，
    // 本行不解引用。
    unsafe {
      (*stack).push(scope);
    }
    Self { stack, scope }
  }
}

impl Drop for StackPusher {
  fn drop(&mut self) {
    if !self.stack.is_null() {
      // Safety: stack 在守卫存活期内有效（unsafe fn new 的调用方契约），且此处
      // 已判空；RAII 配对保证守卫按 LIFO 释放——debug_assert 校验栈顶正是本
      // 守卫压入的 scope，即自 push 后无人越顶改动该 Vec，last/pop 的瞬态
      // 借用无别名冲突；pop 只弹出本守卫压入的元素，不触碰其余额外内容。
      unsafe {
        debug_assert_eq!((*self.stack).last().copied(), Some(self.scope));
        (*self.stack).pop();
      }
    }
  }
}
