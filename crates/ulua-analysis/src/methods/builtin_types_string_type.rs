use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

impl BuiltinTypes {
  pub fn string_type(&self) -> TypeId {
    self.string_type
  }
}
