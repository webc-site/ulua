use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::occurs_check_result::OccursCheckResult,
  functions::{follow_type_pack::follow_type_pack_id, get_type_pack::get_type_pack_id},
  records::{
    blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack, type_pack::TypePack,
  },
  type_aliases::{error_type_pack::ErrorTypePack, type_pack_id::TypePackId},
};

pub(crate) fn occurs_check_type_pack_id_type_pack_id(
  needle: TypePackId,
  haystack: TypePackId,
) -> OccursCheckResult {
  let needle = unsafe { follow_type_pack_id(needle) };
  let mut haystack = unsafe { follow_type_pack_id(haystack) };

  LUAU_ASSERT!(
    get_type_pack_id::<FreeTypePack>(needle).is_some()
      || get_type_pack_id::<BlockedTypePack>(needle).is_some()
  );

  if get_type_pack_id::<ErrorTypePack>(needle).is_some() {
    return OccursCheckResult::Pass;
  }

  while get_type_pack_id::<ErrorTypePack>(haystack).is_none() {
    if needle == haystack {
      return OccursCheckResult::Fail;
    }

    if let Some(a) = get_type_pack_id::<TypePack>(haystack)
      && let Some(tail) = a.tail
    {
      haystack = unsafe { follow_type_pack_id(tail) };
      continue;
    }

    break;
  }

  OccursCheckResult::Pass
}
