use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{dfg_scope::DfgScope, push_scope::PushScope},
  type_aliases::scope_stack::ScopeStack,
};

impl PushScope {
  pub fn new(_stack: &mut ScopeStack, _scope: *mut DfgScope) -> Self {
    // `scope` should never be `nullptr` here.
    LUAU_ASSERT!(!_scope.is_null());

    let previous_size = _stack.len();
    _stack.push(_scope);

    PushScope {
      stack: _stack,
      previous_size,
    }
  }

  pub fn pop(&mut self) {
    if self.previous_size == usize::MAX {
      return;
    }

    // If somehow this stack has _shrunk_ to be smaller than we expect,
    // something very strange has happened.
    // Safety: `self.stack` 由 `PushScope::new` 从调用方（DataFlowGraphBuilder 等）
    // 借出的 `&mut ScopeStack` 写入，RAII 约定保证借用方的栈比本对象长寿；此处
    // 共享借用只读 `len` 做断言，随即随语句结束。
    let stack_ref = unsafe { &*self.stack };
    LUAU_ASSERT!(stack_ref.len() > self.previous_size);

    // Safety: 上一行共享借用已失效，此刻无其它活跃借用，`&mut` 重建唯一——
    // builder 在本作用域挂起对栈的其它操作、单线程无别名冲突。
    let stack_mut = unsafe { &mut *self.stack };
    while stack_mut.len() > self.previous_size {
      stack_mut.pop();
    }
    self.previous_size = usize::MAX;
  }
}
