use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn error_recovery_type_scope_ptr(&mut self, _scope: &ScopePtr) -> TypeId {
    unsafe { (*self.builtin_types).error_type }
  }
}
