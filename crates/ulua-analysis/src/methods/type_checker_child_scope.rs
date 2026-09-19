//! @interface-stub
use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  records::{module::Module, scope::Scope, type_checker::TypeChecker},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn child_scope(&mut self, parent: &ScopePtr, location: &Location) -> ScopePtr {
    let mut scope_value = Scope::new(parent, 0);
    scope_value.level = parent.level;
    scope_value.vararg_pack = parent.vararg_pack;
    scope_value.location = *location;
    scope_value.return_type = parent.return_type;

    let scope = Arc::new(scope_value);

    unsafe {
      let parent_mut = Arc::as_ptr(parent) as *mut Scope;
      (*parent_mut)
        .children
        .push(Arc::as_ptr(&scope) as *mut Scope);

      let module = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
      (*module).scopes.push((*location, scope.clone()));
    }

    scope
  }
}
