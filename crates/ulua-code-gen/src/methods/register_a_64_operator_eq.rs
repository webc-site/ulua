use crate::records::register_a_64::RegisterA64;

impl RegisterA64 {
  #[inline]
  pub const fn register_a_64_operator_eq(&self, rhs: RegisterA64) -> bool {
    self.bits == rhs.bits
  }
}
