use crate::records::{function_argument::FunctionArgument, function_type::FunctionType};

impl FunctionType {
  pub fn arg_names(&self) -> &[Option<FunctionArgument>] {
    &self.arg_names
  }
}
