use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckedFunctionIncorrectArgs {
  pub(crate) function_name: String,
  pub(crate) expected: usize,
  pub(crate) actual: usize,
}

impl CheckedFunctionIncorrectArgs {
  pub const fn new(function_name: String, expected: usize, actual: usize) -> Self {
    Self {
      function_name,
      expected,
      actual,
    }
  }
}

impl CheckedFunctionIncorrectArgs {
  pub fn function_name(&self) -> &str {
    &self.function_name
  }

  pub fn expected(&self) -> usize {
    self.expected
  }

  pub fn actual(&self) -> usize {
    self.actual
  }
}
