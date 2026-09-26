use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
  pub fn is_truthy(&self) -> bool {
    !self.is_falsy()
  }
}
