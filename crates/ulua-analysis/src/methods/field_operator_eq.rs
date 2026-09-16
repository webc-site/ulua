use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::field::Field;

impl Field {
  #[inline]
  pub fn operator_eq(&self, rhs: &Field) -> bool {
    LUAU_ASSERT!(self.parent.is_some() && rhs.parent.is_some());
    self.key == rhs.key
      && (self.parent == rhs.parent || self.parent.as_ref() == rhs.parent.as_ref())
  }
}
