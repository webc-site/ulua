use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn rw_type_id(ty: TypeId) -> Self {
    Self::rw_type_id_type_id(ty, ty)
  }
}
