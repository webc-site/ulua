use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::weird_iter::WeirdIter, type_aliases::type_id::TypeId};

impl WeirdIter {
  pub fn weird_iter_operator_deref(&mut self) -> &mut TypeId {
    LUAU_ASSERT!(self.weird_iter_good());
    unsafe {
      let pack = &mut *self.pack;
      &mut pack.head[self.index]
    }
  }
}
