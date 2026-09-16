use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailedToCompile {
  pub(crate) function_name: String,
  pub(crate) compile_error: String,
}

impl FailedToCompile {
  pub const fn new(function_name: String, compile_error: String) -> Self {
    Self {
      function_name,
      compile_error,
    }
  }
}

impl FailedToCompile {
  pub fn function_name(&self) -> &str {
    &self.function_name
  }

  pub fn compile_error(&self) -> &str {
    &self.compile_error
  }
}
