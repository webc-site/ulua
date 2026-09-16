use crate::records::type_error::TypeError;

impl TypeError {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypeError) -> bool {
    self.location == rhs.location && self.data == rhs.data
  }
}
