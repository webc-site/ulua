use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

impl BuiltinTypes {
  pub fn number_type(&self) -> TypeId {
    self.number_type
  }
}
