use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn writeonly(ty: TypeId) -> Self {
    Property {
      read_ty: None,
      write_ty: Some(ty),
      ..Default::default()
    }
  }
}
