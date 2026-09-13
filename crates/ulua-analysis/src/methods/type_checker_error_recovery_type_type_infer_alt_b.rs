use crate::{records::type_checker::TypeChecker, type_aliases::type_id::TypeId};

impl TypeChecker {
  pub fn error_recovery_type_type_id(&mut self, guess: TypeId) -> TypeId {
    unsafe { (*self.builtin_types).error_recovery_type(guess) }
  }
}
