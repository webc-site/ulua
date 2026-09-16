use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserDefinedTypeFunctionError {
  pub(crate) message: String,
}

impl UserDefinedTypeFunctionError {
  pub const fn new(message: String) -> Self {
    Self { message }
  }
}

impl UserDefinedTypeFunctionError {
  pub fn message(&self) -> &str {
    &self.message
  }
}
