use crate::records::register_x_64::RegisterX64;

impl RegisterX64 {
  #[inline]
  pub fn register_x_64_operator_eq(&self, rhs: RegisterX64) -> bool {
    self.size() == rhs.size() && self.index() == rhs.index()
  }
}
