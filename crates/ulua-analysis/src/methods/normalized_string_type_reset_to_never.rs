use crate::records::normalized_string_type::NormalizedStringType;

impl NormalizedStringType {
  pub fn reset_to_never(&mut self) {
    self.is_cofinite = false;
    self.singletons.clear();
  }
}
