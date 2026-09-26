use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn type_deprecated(&self) -> TypeId {
    LUAU_ASSERT!(self.read_ty.is_some());
    // 紧邻 LUAU_ASSERT 蕴含 Some（对应 cpp `LUAU_ASSERT(readTy); return *readTy;`）。
    self
      .read_ty
      .expect("紧邻 LUAU_ASSERT(read_ty.is_some()) 蕴含")
  }
}
