use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError {
  pub(crate) message: String,
}

impl SyntaxError {
  pub const fn new(message: String) -> Self {
    Self { message }
  }
}

impl SyntaxError {
  pub fn message(&self) -> &str {
    &self.message
  }
}
