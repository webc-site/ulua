use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn fresh_type_scope_ptr(&mut self, scope: ScopePtr) -> TypeId {
    self.fresh_type_type_level(scope.level)
  }
}
