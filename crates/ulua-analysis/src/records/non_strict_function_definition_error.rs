use alloc::string::String;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonStrictFunctionDefinitionError {
  pub(crate) function_name: String,
  pub(crate) argument: String,
  pub(crate) argument_type: TypeId,
}

impl NonStrictFunctionDefinitionError {
  pub const fn new(function_name: String, argument: String, argument_type: TypeId) -> Self {
    Self {
      function_name,
      argument,
      argument_type,
    }
  }
}

impl NonStrictFunctionDefinitionError {
  pub fn function_name(&self) -> &str {
    &self.function_name
  }

  pub fn argument(&self) -> &str {
    &self.argument
  }

  pub fn argument_type(&self) -> TypeId {
    self.argument_type
  }
}
