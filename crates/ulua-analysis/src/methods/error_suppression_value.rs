use crate::{enums::value::Value, records::error_suppression::ErrorSuppression};

impl ErrorSuppression {
  #[inline]
  pub fn error_suppression_value(&self) -> Value {
    self.value
  }
}
