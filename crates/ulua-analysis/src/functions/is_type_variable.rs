use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    blocked_type::BlockedType, free_type::FreeType, generic_type::GenericType,
    pending_expansion_type::PendingExpansionType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_type_variable(ty: TypeId) -> bool {
  !get_type_id::<FreeType>(ty).is_none()
    || !get_type_id::<GenericType>(ty).is_none()
    || !get_type_id::<BlockedType>(ty).is_none()
    || !get_type_id::<PendingExpansionType>(ty).is_none()
}
