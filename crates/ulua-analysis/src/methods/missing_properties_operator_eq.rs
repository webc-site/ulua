use crate::records::missing_properties::MissingProperties;

impl MissingProperties {
  #[inline]
  pub fn operator_eq(&self, rhs: &MissingProperties) -> bool {
    self.super_type == rhs.super_type
      && self.sub_type == rhs.sub_type
      && self.properties == rhs.properties
      && self.context == rhs.context
  }
}
