use ulua_ast::records::location::Location;

use crate::{
  records::{scope::Scope, type_checker::TypeChecker},
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

    // 公共尾段（注册/挂父/登记 module.scopes）见 register_child_scope。
    self.register_child_scope(parent, scope_value, location)
  }
}
