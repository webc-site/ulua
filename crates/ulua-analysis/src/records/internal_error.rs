use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InternalError {
  pub(crate) message: String,
}

impl InternalError {
  pub const fn new(message: String) -> Self {
    Self { message }
  }
}

impl InternalError {
  pub fn message(&self) -> &str {
    &self.message
  }
}
