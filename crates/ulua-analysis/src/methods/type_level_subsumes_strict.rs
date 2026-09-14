use crate::records::type_level::TypeLevel;

impl TypeLevel {
  pub fn subsumes_strict(&self, rhs: &TypeLevel) -> bool {
    if self.level == rhs.level && self.sub_level == rhs.sub_level {
      false
    } else {
      self.subsumes(rhs)
    }
  }
}
