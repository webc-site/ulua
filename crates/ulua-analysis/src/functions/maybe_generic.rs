use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{begin_type::begin_intersection_type, follow_type, get_type, is_generic::is_generic},
  records::{free_type::FreeType, intersection_type::IntersectionType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

pub fn maybe_generic(ty: TypeId) -> bool {
  LUAU_ASSERT!(!fflag::LuauInstantiateInSubtyping.get());

  let ty = follow_type::follow(ty);

  if get_type::get::<FreeType>(ty).is_some() {
    return true;
  }

  if get_type::get::<TableType>(ty).is_some() {
    // TODO: recurse on table types CLI-39914
    return true;
  }

  if let Some(itv) = get_type::get::<IntersectionType>(ty) {
    // C++ `std::any_of(begin(itv), end(itv), maybeGeneric)` — IntersectionTypeIterator
    // 防环展平并 follow,裸遍历 parts 会漏掉嵌套 intersection。
    return begin_intersection_type(itv).any(maybe_generic);
  }

  is_generic(ty)
}
