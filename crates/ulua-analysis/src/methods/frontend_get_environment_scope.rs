use alloc::{string::String, sync::Arc};
use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{frontend::Frontend, scope::Scope, scope_registry::register_scope},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl Frontend {
  pub fn get_environment_scope(&self, environment_name: String) -> ScopePtr {
    if let Some(scope) = self.environments.get(&environment_name) {
      return scope.clone();
    }

    LUAU_ASSERT!(false, "environment doesn't exist");
    let scope = Arc::new(Scope::scope_type_pack_id(null()));
    register_scope(&scope);
    scope
  }
}
