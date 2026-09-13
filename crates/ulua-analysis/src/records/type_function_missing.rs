use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunctionMissing {
  pub(crate) function_name: String,
}

impl TypeFunctionMissing {
  pub const fn new(function_name: String) -> Self {
    Self { function_name }
  }
}

impl TypeFunctionMissing {
  pub fn function_name(&self) -> &str {
    &self.function_name
  }
}
