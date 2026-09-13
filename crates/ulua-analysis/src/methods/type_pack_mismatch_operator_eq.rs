use crate::records::type_pack_mismatch::TypePackMismatch;

impl TypePackMismatch {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypePackMismatch) -> bool {
    self.wanted_tp == rhs.wanted_tp && self.given_tp == rhs.given_tp
  }
}
