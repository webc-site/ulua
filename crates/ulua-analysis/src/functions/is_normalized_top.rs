use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{any_type::AnyType, never_type::NeverType, unknown_type::UnknownType},
  type_aliases::type_id::TypeId,
};

pub fn is_normalized_top(ty: TypeId) -> bool {
  !get_type_id::<NeverType>(ty).is_none()
    || !get_type_id::<AnyType>(ty).is_none()
    || !get_type_id::<UnknownType>(ty).is_none()
}
