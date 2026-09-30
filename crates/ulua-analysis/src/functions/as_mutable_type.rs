use crate::{records::r#type::Type, type_aliases::type_id::TypeId};

pub fn as_mutable_type_id(ty: TypeId) -> *mut Type {
  ty as *mut Type
}
