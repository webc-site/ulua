use crate::records::missing_union_property::MissingUnionProperty;

impl MissingUnionProperty {
  #[inline]
  pub fn operator_eq(&self, rhs: &MissingUnionProperty) -> bool {
    self.missing == rhs.missing && self.r#type == rhs.r#type && self.key == rhs.key
  }
}
