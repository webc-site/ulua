use crate::records::normalized_function_type::NormalizedFunctionType;

impl NormalizedFunctionType {
  pub fn reset_to_top(&mut self) {
    self.is_top = true;
    self.parts.clear();
  }
}
