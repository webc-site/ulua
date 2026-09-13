use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::function_type::FunctionType,
  type_aliases::type_id::TypeId,
};

pub fn is_generic(ty: TypeId) -> bool {
  let followed = follow_type_id(ty);
  get_type_id::<FunctionType>(followed)
    .is_some_and(|ftv| !ftv.generics.is_empty() || !ftv.generic_packs.is_empty())
}
