use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_optional::is_optional},
  records::free_type::FreeType,
  type_aliases::type_id::TypeId,
};

pub fn is_optional_or_free(ty: TypeId) -> bool {
  let followed = follow_type_id(ty);
  is_optional(followed) || !get_type_id::<FreeType>(followed).is_none()
}
