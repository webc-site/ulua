use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::{alias, alias_ref},
    dfg_scope::DfgScope,
    push_scope::PushScope,
  },
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
      previous_size: Some(previous_size),
    }
  }

  pub fn pop(&mut self) {
    let Some(previous_size) = self.previous_size.take() else {
      return;
    };

    // If somehow this stack has _shrunk_ to be smaller than we expect,
    // something very strange has happened.
    let stack_ref = alias_ref(self.stack);
    LUAU_ASSERT!(stack_ref.len() > previous_size);

    let stack_mut = alias(self.stack);
    stack_mut.truncate(previous_size);
  }
}
