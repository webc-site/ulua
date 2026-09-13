use crate::records::unification_too_complex::UnificationTooComplex;

impl UnificationTooComplex {
  #[inline]
  pub fn operator_eq(&self, _other: &UnificationTooComplex) -> bool {
    true
  }
}
