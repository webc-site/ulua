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
}
