use alloc::string::String;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckedFunctionCallError {
  pub(crate) expected: TypeId,
  pub(crate) passed: TypeId,
  pub(crate) checked_function_name: String,
  pub(crate) argument_index: usize,
}

impl CheckedFunctionCallError {
  pub const fn new(
    expected: TypeId,
    passed: TypeId,
    checked_function_name: String,
    argument_index: usize,
  ) -> Self {
    Self {
      expected,
      passed,
      checked_function_name,
      argument_index,
    }
  }

  pub fn expected(&self) -> TypeId {
    self.expected
  }

  pub fn passed(&self) -> TypeId {
    self.passed
  }

  pub fn checked_function_name(&self) -> &str {
    &self.checked_function_name
  }

  pub fn argument_index(&self) -> usize {
    self.argument_index
  }
}
