use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn rw_type_id_type_id(read: TypeId, write: TypeId) -> Self {
    Property {
      read_ty: Some(read),
      write_ty: Some(write),
      ..Default::default()
    }
  }
}
