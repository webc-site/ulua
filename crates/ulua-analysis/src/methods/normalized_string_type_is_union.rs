use crate::records::normalized_string_type::NormalizedStringType;

impl NormalizedStringType {
  pub fn is_union(&self) -> bool {
    !self.is_cofinite
  }
}
