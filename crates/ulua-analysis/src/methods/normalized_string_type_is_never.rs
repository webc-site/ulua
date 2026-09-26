use crate::records::normalized_string_type::NormalizedStringType;

impl NormalizedStringType {
  pub fn is_never(&self) -> bool {
    !self.is_cofinite && self.singletons.is_empty()
  }
}
