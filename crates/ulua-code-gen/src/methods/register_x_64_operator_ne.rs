use crate::records::register_x_64::RegisterX64;

impl RegisterX64 {
  #[inline]
  pub const fn register_x_64_operator_ne(&self, rhs: RegisterX64) -> bool {
    self.bits != rhs.bits
  }
}
