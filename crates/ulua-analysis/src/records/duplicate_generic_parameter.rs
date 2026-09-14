use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuplicateGenericParameter {
  pub(crate) parameter_name: String,
}

impl DuplicateGenericParameter {
  pub const fn new(parameter_name: String) -> Self {
    Self { parameter_name }
  }
}

impl DuplicateGenericParameter {
  pub fn parameter_name(&self) -> &str {
    &self.parameter_name
  }
}
