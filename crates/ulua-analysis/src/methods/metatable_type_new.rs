use crate::{records::metatable_type::MetatableType, type_aliases::type_id::TypeId};

impl MetatableType {
  pub fn new(table: TypeId, metatable: TypeId) -> Self {
    Self {
      table,
      metatable,
      synthetic_name: None,
    }
  }
}
