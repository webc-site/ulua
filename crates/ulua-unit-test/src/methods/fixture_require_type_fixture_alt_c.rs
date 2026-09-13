use alloc::sync::Arc;

use ulua_analysis::{records::module::Module, type_aliases::type_id::TypeId};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn require_type_module_ptr_string(&mut self, module: &Module, name: &str) -> TypeId {
    let scope = module.get_module_scope();
    let scope = Arc::as_ptr(&scope) as *mut _;
    unsafe { self.require_type_scope_ptr_string(scope, name) }
  }
}
