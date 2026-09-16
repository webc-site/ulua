use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

impl TypeChecker {
  pub fn fresh_type_pack_scope_ptr(&mut self, scope: ScopePtr) -> TypePackId {
    self.fresh_type_pack_type_level(scope.level)
  }
}
