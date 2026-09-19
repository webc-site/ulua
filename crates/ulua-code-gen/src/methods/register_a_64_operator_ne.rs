use crate::records::register_a_64::RegisterA64;

impl RegisterA64 {
  #[inline]
  pub const fn register_a_64_operator_ne(&self, rhs: RegisterA64) -> bool {
    self.bits != rhs.bits
  }
}

impl PartialEq<RegisterA64> for &RegisterA64 {
  #[inline]
  fn eq(&self, other: &RegisterA64) -> bool {
    self.bits == other.bits
  }
}
