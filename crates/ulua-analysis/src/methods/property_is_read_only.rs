use crate::records::property_type::Property;

impl Property {
  #[inline]
  pub fn is_read_only(&self) -> bool {
    self.read_ty.is_some() && self.write_ty.is_none()
  }
}
