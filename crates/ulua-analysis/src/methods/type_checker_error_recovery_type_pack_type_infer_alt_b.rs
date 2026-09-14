use crate::{records::type_checker::TypeChecker, type_aliases::type_pack_id::TypePackId};

impl TypeChecker {
  pub fn error_recovery_type_pack_type_pack_id(&mut self, guess: TypePackId) -> TypePackId {
    unsafe { (*self.builtin_types).error_recovery_type_pack(guess) }
  }
}
