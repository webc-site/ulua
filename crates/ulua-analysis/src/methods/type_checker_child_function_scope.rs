use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{scope::Scope, scope_registry::register_scope, type_checker::TypeChecker},
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
    // 注册发放本 scope 的句柄，父 children 只存句柄（裸地址仅经注册点入系统）。
    let scope_id = register_scope(&scope);

    unsafe {
      let parent_mut = arc_as_mut(parent);
      (*parent_mut).children.push(scope_id);

      let module = arc_as_mut(self.expect_current_module());
      (*module).scopes.push((*location, scope.clone()));
    }

    scope
  }
}
