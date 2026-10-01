use crate::{
  functions::{follow_type, get_type},
  records::function_type::FunctionType,
  type_aliases::type_id::TypeId,
};

pub fn is_generic(ty: TypeId) -> bool {
  let followed = follow_type::follow(ty);
  get_type::get::<FunctionType>(followed)
    .is_some_and(|ftv| !ftv.generics.is_empty() || !ftv.generic_packs.is_empty())
}
