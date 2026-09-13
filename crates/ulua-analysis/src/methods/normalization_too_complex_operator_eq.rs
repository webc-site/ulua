use crate::records::normalization_too_complex::NormalizationTooComplex;

impl NormalizationTooComplex {
  #[inline]
  pub fn operator_eq(&self, _other: &NormalizationTooComplex) -> bool {
    true
  }
}
