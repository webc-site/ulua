use crate::records::iterator::Iterator;

impl Iterator {
  #[inline]
  pub fn operator_eq(&self, rhs: &Iterator) -> bool {
    self.cl == rhs.cl && self.index == rhs.index
  }
}
