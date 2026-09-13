use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;

impl IncorrectGenericParameterCount {
  pub fn name(&self) -> &str {
    &self.name
  }
}
