use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_pack_id::TypePackId};

impl BuiltinTypes {
  pub fn error_recovery_type_pack(&self, guess: TypePackId) -> TypePackId {
    guess
  }
}
