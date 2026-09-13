use crate::records::code_too_complex::CodeTooComplex;

impl CodeTooComplex {
  #[inline]
  pub fn operator_eq(&self, _other: &CodeTooComplex) -> bool {
    true
  }
}
