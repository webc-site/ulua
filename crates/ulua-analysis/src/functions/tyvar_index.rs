use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{blocked_type::BlockedType, free_type::FreeType, generic_type::GenericType},
  type_aliases::type_id::TypeId,
};

pub fn tyvar_index(ty: TypeId) -> i32 {
  if let Some(gtv) = get_type_id::<GenericType>(ty).as_ref() {
    gtv.index
  } else if let Some(ftv) = get_type_id::<FreeType>(ty).as_ref() {
    ftv.index
  } else if let Some(btv) = get_type_id::<BlockedType>(ty).as_ref() {
    btv.index
  } else {
    0
  }
}
