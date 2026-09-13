use crate::records::iterator::Iterator;

impl Iterator {
  #[inline]
  pub fn operator_ne(&self, rhs: &Iterator) -> bool {
    !self.operator_eq(rhs)
  }
}
