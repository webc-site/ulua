use core::ptr::null_mut;

use crate::{
  functions::get_level_type::get_level, records::type_level::TypeLevel,
  type_aliases::type_id::TypeId,
};
pub fn get_mutable_level(ty: TypeId) -> *mut TypeLevel {
  if let Some(level) = get_level(ty) {
    level as *const TypeLevel as *mut TypeLevel
  } else {
    null_mut()
  }
}
