use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuntimeError {
  pub(crate) message: String,
}

impl RuntimeError {
  pub const fn new(message: String) -> Self {
    Self { message }
  }
}

impl RuntimeError {
  pub fn message(&self) -> &str {
    &self.message
  }
}
