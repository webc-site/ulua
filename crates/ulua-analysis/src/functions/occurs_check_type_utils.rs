use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::occurs_check_result::OccursCheckResult,
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type, follow_type_pack, get_type, get_type_pack,
  },
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    intersection_type::IntersectionType, pending_expansion_type::PendingExpansionType,
    type_pack::TypePack, union_type::UnionType,
  },
  type_aliases::{error_type_pack::ErrorTypePack, type_id::TypeId, type_pack_id::TypePackId},
};

pub fn occurs_check_type_id_type_id(needle: TypeId, haystack: TypeId) -> bool {
  LUAU_ASSERT!(
    get_type::get::<BlockedType>(needle).is_some()
      || get_type::get::<PendingExpansionType>(needle).is_some()
  );

  let haystack = follow_type::follow(haystack);

  let check_haystack =
    |haystack: TypeId| -> bool { occurs_check_type_id_type_id(needle, haystack) };

  if needle == haystack {
    return true;
  }

  // C++ `std::any_of(begin(ut), end(ut), ...)` — TypeIterator 防环展平并 follow,
  // 裸遍历对自嵌套类型会漏检甚至无限递归。
  if let Some(ut) = get_type::get::<UnionType>(haystack) {
    return begin_union_type(ut).any(&check_haystack);
  }

  if let Some(it) = get_type::get::<IntersectionType>(haystack) {
    return begin_intersection_type(it).any(check_haystack);
  }

  false
}

pub(crate) fn occurs_check_type_pack_id_type_pack_id(
  needle: TypePackId,
  haystack: TypePackId,
) -> OccursCheckResult {
  let needle = follow_type_pack::follow(needle);
  let mut haystack = follow_type_pack::follow(haystack);

  LUAU_ASSERT!(
    get_type_pack::get::<FreeTypePack>(needle).is_some()
      || get_type_pack::get::<BlockedTypePack>(needle).is_some()
  );

  if get_type_pack::get::<ErrorTypePack>(needle).is_some() {
    return OccursCheckResult::Pass;
  }

  while get_type_pack::get::<ErrorTypePack>(haystack).is_none() {
    if needle == haystack {
      return OccursCheckResult::Fail;
    }

    if let Some(a) = get_type_pack::get::<TypePack>(haystack)
      && let Some(tail) = a.tail
    {
      haystack = follow_type_pack::follow(tail);
      continue;
    }

    break;
  }

  OccursCheckResult::Pass
}
