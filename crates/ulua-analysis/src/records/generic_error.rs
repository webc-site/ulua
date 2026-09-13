use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericError {
  pub(crate) message: String,
}

impl GenericError {
  pub const fn new(message: String) -> Self {
    Self { message }
  }
}

impl GenericError {
  pub fn message(&self) -> &str {
    &self.message
  }
}
