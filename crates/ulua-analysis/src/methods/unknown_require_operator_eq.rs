use crate::records::unknown_require::UnknownRequire;

impl UnknownRequire {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnknownRequire) -> bool {
    self.module_path == rhs.module_path
  }
}
