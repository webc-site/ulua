use alloc::string::String;

use crate::{records::metatable_type::MetatableType, type_aliases::type_id::TypeId};

impl MetatableType {
  pub fn new_named(table: TypeId, metatable: TypeId, synthetic_name: String) -> Self {
    Self {
      table,
      metatable,
      synthetic_name: Some(synthetic_name),
    }
  }
}
