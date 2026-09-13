use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    blocked_type::BlockedType, intersection_type::IntersectionType,
    pending_expansion_type::PendingExpansionType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

pub fn occurs_check_type_id_type_id(needle: TypeId, haystack: TypeId) -> bool {
  LUAU_ASSERT!(
    get_type_id::<BlockedType>(needle).is_some()
      || get_type_id::<PendingExpansionType>(needle).is_some()
  );

  let haystack = follow_type_id(haystack);

  let check_haystack =
    |haystack: TypeId| -> bool { occurs_check_type_id_type_id(needle, haystack) };

  if needle == haystack {
    return true;
  }

  if let Some(ut) = get_type_id::<UnionType>(haystack) {
    return ut.options.iter().any(|&option| check_haystack(option));
  }

  if let Some(it) = get_type_id::<IntersectionType>(haystack) {
    return it.parts.iter().any(|&part| check_haystack(part));
  }

  false
}
