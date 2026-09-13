use crate::records::count_mismatch::CountMismatch;

impl CountMismatch {
  #[inline]
  pub fn operator_eq(&self, rhs: &CountMismatch) -> bool {
    self.expected == rhs.expected
      && self.maximum == rhs.maximum
      && self.actual == rhs.actual
      && self.context == rhs.context
      && self.function == rhs.function
  }
}
