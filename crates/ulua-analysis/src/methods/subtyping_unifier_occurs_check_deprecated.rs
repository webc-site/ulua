use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::occurs_check_result::OccursCheckResult,
  functions::{
    follow_type_pack::follow_type_pack_id, get_mutable_type_pack::get_mutable_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::{
    free_type_pack::FreeTypePack, subtyping_unifier::SubtypingUnifier, type_pack::TypePack,
  },
  type_aliases::{error_type_pack::ErrorTypePack as ErrorTypePackAlias, type_pack_id::TypePackId},
};
impl SubtypingUnifier {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn occurs_check_deprecated(
    &self,
    needle: TypePackId,
    haystack: TypePackId,
  ) -> OccursCheckResult {
    let needle_followed = unsafe { follow_type_pack_id(needle) };
    let haystack_followed = unsafe { follow_type_pack_id(haystack) };

    if get_mutable_type_pack_id::<ErrorTypePackAlias>(needle_followed).is_some() {
      return OccursCheckResult::Pass;
    }

    if get_mutable_type_pack_id::<FreeTypePack>(needle_followed).is_none() {
      LUAU_ASSERT!(false, "Expected needle pack to be free");
    }

    let mut current_haystack = haystack_followed;
    while get_mutable_type_pack_id::<ErrorTypePackAlias>(current_haystack).is_none() {
      if needle_followed == current_haystack {
        return OccursCheckResult::Fail;
      }

      if let Some(pack) = get_type_pack_id::<TypePack>(current_haystack)
        && let Some(tail) = pack.tail
      {
        current_haystack = unsafe { follow_type_pack_id(tail) };
        continue;
      }

      break;
    }

    OccursCheckResult::Pass
  }
}
