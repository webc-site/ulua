use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;

impl IncorrectGenericParameterCount {
  pub fn actual_parameters(&self) -> usize {
    self.actual_parameters
  }
}
