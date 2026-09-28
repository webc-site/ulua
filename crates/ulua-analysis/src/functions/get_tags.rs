use alloc::{string::String, vec::Vec};

use crate::{
  functions::{follow_type, get_mutable_type},
  records::{extern_type::ExternType, function_type::FunctionType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn get_tags(ty: TypeId) -> Option<&'static mut Vec<String>> {
  let ty = follow_type::follow(ty);

  if let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(ty) {
    Some(&mut ftv.tags)
  } else if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(ty) {
    Some(&mut ttv.tags)
  } else if let Some(etv) = get_mutable_type::get_mutable::<ExternType>(ty) {
    Some(&mut etv.tags)
  } else {
    None
  }
}
