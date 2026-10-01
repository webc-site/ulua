use crate::{
  functions::{follow_type, get_type, is_optional::is_optional},
  records::free_type::FreeType,
  type_aliases::type_id::TypeId,
};

pub fn is_optional_or_free(ty: TypeId) -> bool {
  let followed = follow_type::follow(ty);
  is_optional(followed) || get_type::get::<FreeType>(followed).is_some()
}
