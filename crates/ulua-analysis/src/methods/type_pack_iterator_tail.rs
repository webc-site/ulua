use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::type_pack_iterator::TypePackIterator, type_aliases::type_pack_id::TypePackId,
};

impl TypePackIterator {
  pub fn tail(&self) -> Option<TypePackId> {
    LUAU_ASSERT!(self.tp.is_null());
    if !self.current_type_pack.is_null() {
      Some(self.current_type_pack)
    } else {
      None
    }
  }
}
