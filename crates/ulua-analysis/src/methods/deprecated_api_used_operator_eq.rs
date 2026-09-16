use crate::records::deprecated_api_used::DeprecatedApiUsed;

impl DeprecatedApiUsed {
  #[inline]
  pub fn operator_eq(&self, rhs: &DeprecatedApiUsed) -> bool {
    self.symbol == rhs.symbol && self.use_instead == rhs.use_instead
  }
}
