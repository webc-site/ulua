use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

impl BuiltinTypes {
  pub fn boolean_type(&self) -> TypeId {
    self.boolean_type
  }
}
