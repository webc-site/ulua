use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

impl BuiltinTypes {
  pub fn error_recovery_type(&self, guess: TypeId) -> TypeId {
    guess
  }
}
