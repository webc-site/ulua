use crate::{
  functions::get_type_alt_j::get_type_id,
  records::never_type::NeverType,
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

pub fn is_normalized_error(ty: TypeId) -> bool {
  !get_type_id::<NeverType>(ty).is_none() || !get_type_id::<ErrorType>(ty).is_none()
}
