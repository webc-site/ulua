use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
  pub fn has_strings(&self) -> bool {
    !self.strings.is_never()
  }
}
