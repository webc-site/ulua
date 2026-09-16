use crate::records::{function_definition::FunctionDefinition, function_type::FunctionType};

impl FunctionType {
  pub fn definition(&self) -> Option<&FunctionDefinition> {
    self.definition.as_ref()
  }
}
