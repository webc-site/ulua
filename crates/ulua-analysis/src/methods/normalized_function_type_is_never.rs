use crate::records::normalized_function_type::NormalizedFunctionType;

impl NormalizedFunctionType {
  pub fn is_never(&self) -> bool {
    !self.is_top && self.parts.empty()
  }
}
