use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

impl BuiltinTypes {
  pub fn empty_table_type(&self) -> TypeId {
    self.empty_table_type
  }
}
