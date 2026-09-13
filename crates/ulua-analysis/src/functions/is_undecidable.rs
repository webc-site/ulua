use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{any_type::AnyType, free_type::FreeType},
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

/// C++ `static bool isUndecidable(TypeId ty)`.
pub fn is_undecidable(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);
  !get_type_id::<AnyType>(ty).is_none()
    || !get_type_id::<ErrorType>(ty).is_none()
    || !get_type_id::<FreeType>(ty).is_none()
}
