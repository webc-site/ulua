use crate::records::unknown_property::UnknownProperty;

impl UnknownProperty {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnknownProperty) -> bool {
    self.table() == rhs.table() && self.key() == rhs.key()
  }
}
