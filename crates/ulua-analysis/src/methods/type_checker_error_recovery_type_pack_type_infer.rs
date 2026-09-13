use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

impl TypeChecker {
  pub fn error_recovery_type_pack_scope_ptr(&mut self, _scope: ScopePtr) -> TypePackId {
    unsafe { (*self.builtin_types).error_type_pack }
  }
}
