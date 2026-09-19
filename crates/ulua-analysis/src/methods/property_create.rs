use crate::{records::property_type::Property, type_aliases::type_id::TypeId};

impl Property {
  pub fn create(read: Option<TypeId>, write: Option<TypeId>) -> Self {
    if let Some(read_ty) = read {
      if let Some(write_ty) = write {
        return Property::rw_type_id_type_id(read_ty, write_ty);
      } else {
        return Property::readonly(read_ty);
      }
    }

    if let Some(write_ty) = write {
      return Property::writeonly(write_ty);
    }

    ulua_common::LUAU_ASSERT!(read.is_some() && write.is_some());
    Property::rw_type_id_type_id(read.unwrap(), write.unwrap())
  }
}
