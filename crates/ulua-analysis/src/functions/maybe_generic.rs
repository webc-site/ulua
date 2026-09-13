use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_generic::is_generic},
  records::{free_type::FreeType, intersection_type::IntersectionType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn maybe_generic(ty: TypeId) -> bool {
  LUAU_ASSERT!(!FFlag::LuauInstantiateInSubtyping.get());

  let ty = follow_type_id(ty);

  if get_type_id::<FreeType>(ty).is_some() {
    return true;
  }

  if get_type_id::<TableType>(ty).is_some() {
    // TODO: recurse on table types CLI-39914
    return true;
  }

  if let Some(itv) = get_type_id::<IntersectionType>(ty) {
    return itv.parts.iter().any(|&part| maybe_generic(part));
  }

  is_generic(ty)
}
