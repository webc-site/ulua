use crate::records::missing_union_property::MissingUnionProperty;

impl MissingUnionProperty {
  #[inline]
  pub fn operator_eq(&self, rhs: &MissingUnionProperty) -> bool {
    if self.missing.len() != rhs.missing.len() {
      return false;
    }

    for i in 0..self.missing.len() {
      if self.missing[i] != rhs.missing[i] {
        return false;
      }
    }

    self.r#type == rhs.r#type && self.key == rhs.key
  }
}
