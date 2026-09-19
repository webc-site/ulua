use crate::records::property_type_path::Property;

impl Property {
  #[inline]
  pub fn operator_eq(&self, rhs: &Self) -> bool {
    self.name == rhs.name && self.is_read == rhs.is_read
  }
}
