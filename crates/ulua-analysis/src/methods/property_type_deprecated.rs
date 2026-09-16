use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn type_deprecated(&self) -> TypeId {
    LUAU_ASSERT!(self.read_ty.is_some());
    self.read_ty.unwrap()
  }
}
