use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    is_approximately_falsy_type::is_approximately_falsy_type,
  },
  records::negation_type::NegationType,
  type_aliases::type_id::TypeId,
};

pub fn is_approximately_truthy_type(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);
  if let Some(nt) = get_type_id::<NegationType>(ty) {
    return is_approximately_falsy_type(nt.ty);
  }
  false
}
