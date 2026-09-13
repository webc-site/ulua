use crate::records::normalized_string_type::NormalizedStringType;

impl NormalizedStringType {
  pub fn includes(&self, str: &str) -> bool {
    if self.is_string() || (self.is_union() && self.singletons.contains_key(str)) {
      true
    } else {
      self.is_intersection() && !self.singletons.contains_key(str)
    }
  }
}
