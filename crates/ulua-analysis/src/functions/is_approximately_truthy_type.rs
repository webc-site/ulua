use crate::{
  functions::{follow_type, get_type, is_approximately_falsy_type::is_approximately_falsy_type},
  records::negation_type::NegationType,
  type_aliases::type_id::TypeId,
};

pub fn is_approximately_truthy_type(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);
  if let Some(nt) = get_type::get::<NegationType>(ty) {
    return is_approximately_falsy_type(nt.ty);
  }
  false
}
