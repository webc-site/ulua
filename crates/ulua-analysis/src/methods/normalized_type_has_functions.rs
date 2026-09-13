use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
  pub fn has_functions(&self) -> bool {
    !self.functions.is_never()
  }
}
