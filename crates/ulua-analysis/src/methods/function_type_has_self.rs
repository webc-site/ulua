use crate::records::function_type::FunctionType;

impl FunctionType {
  pub fn has_self(&self) -> bool {
    self.has_self
  }
}
