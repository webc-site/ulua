use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::type_pack_iterator::TypePackIterator, type_aliases::type_id::TypeId};

impl TypePackIterator {
  pub fn operator_deref(&self) -> &TypeId {
    LUAU_ASSERT!(!self.tp.is_null());
    unsafe {
      let tp = &*self.tp;
      &tp.head[self.current_index]
    }
  }
}
