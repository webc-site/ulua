use crate::records::syntax_error::SyntaxError;

impl SyntaxError {
  #[inline]
  pub fn operator_eq(&self, rhs: &SyntaxError) -> bool {
    self.message == rhs.message
  }
}
