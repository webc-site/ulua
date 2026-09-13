use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bc_ref::BcRef;

impl<'a, T> BcRef<'a, T> {
  pub fn operator_arrow(&self) -> *mut T {
    LUAU_ASSERT!((self.op.index as usize) < self.vec.len());
    &self.vec[self.op.index as usize] as *const T as *mut T
  }
}
