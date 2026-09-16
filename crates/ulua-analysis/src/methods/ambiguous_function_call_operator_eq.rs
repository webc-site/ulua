use crate::records::ambiguous_function_call::AmbiguousFunctionCall;

impl AmbiguousFunctionCall {
  #[inline]
  pub fn operator_eq(&self, rhs: &AmbiguousFunctionCall) -> bool {
    self.function == rhs.function && self.arguments == rhs.arguments
  }
}
