use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  records::{module::Module, scope::Scope, type_checker::TypeChecker},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeChecker {
  pub fn child_function_scope(
    &mut self,
    parent: &ScopePtr,
    location: &Location,
    sub_level: i32,
  ) -> ScopePtr {
    let mut scope_value = Scope::new(parent, sub_level);
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
