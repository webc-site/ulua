use crate::records::parse_options::ParseOptions;

impl ParseOptions {
  /// Default-initialized parse options.
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }
}
