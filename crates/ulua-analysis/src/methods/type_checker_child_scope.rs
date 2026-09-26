use alloc::sync::Arc;

use ulua_ast::records::location::Location;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{scope::Scope, scope_registry::register_scope, type_checker::TypeChecker},
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
    // 注册发放本 scope 的句柄，父 children 只存句柄（裸地址仅经注册点入系统）。
    let scope_id = register_scope(&scope);

    unsafe {
      let parent_mut = arc_as_mut(parent);
      (*parent_mut).children.push(scope_id);

      let module = arc_as_mut(self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some"));
      (*module).scopes.push((*location, scope.clone()));
    }

    scope
  }
}
