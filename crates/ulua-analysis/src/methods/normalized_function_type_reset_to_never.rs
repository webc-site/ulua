use crate::records::normalized_function_type::NormalizedFunctionType;

impl NormalizedFunctionType {
  pub fn reset_to_never(&mut self) {
    self.is_top = false;
    self.parts.clear();
  }
}
