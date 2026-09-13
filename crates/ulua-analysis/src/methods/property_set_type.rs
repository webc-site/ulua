use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn set_type(&mut self, ty: TypeId) {
    self.read_ty = Some(ty);
    self.write_ty = Some(ty);
  }
}
