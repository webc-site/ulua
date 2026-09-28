use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn readonly(ty: TypeId) -> Self {
    Property {
      read_ty: Some(ty),
      write_ty: None,
      ..Default::default()
    }
  }
}
