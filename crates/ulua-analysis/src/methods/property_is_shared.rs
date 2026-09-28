use crate::records::property_type::Property;

impl Property {
  pub fn is_shared(&self) -> bool {
    self.read_ty.is_some() && self.write_ty.is_some() && self.read_ty == self.write_ty
  }
}
