use crate::{
  functions::{follow_type, get_type},
  records::{any_type::AnyType, free_type::FreeType},
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

/// C++ `static bool isUndecidable(TypeId ty)`.
pub fn is_undecidable(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);
  get_type::get::<AnyType>(ty).is_some()
    || get_type::get::<ErrorType>(ty).is_some()
    || get_type::get::<FreeType>(ty).is_some()
}
