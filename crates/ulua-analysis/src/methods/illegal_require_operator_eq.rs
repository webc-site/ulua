use crate::records::illegal_require::IllegalRequire;

impl IllegalRequire {
  #[inline]
  pub fn operator_eq(&self, rhs: &IllegalRequire) -> bool {
    self.module_name == rhs.module_name && self.reason == rhs.reason
  }
}
