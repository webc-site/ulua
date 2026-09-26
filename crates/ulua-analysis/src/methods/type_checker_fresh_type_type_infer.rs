use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{type_checker::TypeChecker, type_level::TypeLevel},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn fresh_type_scope_ptr(&mut self, scope: ScopePtr) -> TypeId {
    self.fresh_type_type_level(scope.level)
  }

  pub fn fresh_type_type_level(&mut self, level: TypeLevel) -> TypeId {
    unsafe {
      let module = arc_as_mut(self.current_module.as_ref().expect("current_module 由 check_without_recursion_check 入口置入 Some、末尾才 take()，check 调用树内恒为 Some"));
      (*module)
        .internal_types
        .fresh_type_not_null_builtin_types_type_level(self.builtin_types.get(), level)
    }
  }
}
