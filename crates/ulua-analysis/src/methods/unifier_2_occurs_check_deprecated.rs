//! Source: `Analysis/src/Unifier2.cpp:873-906` —
//! `Unifier2::occursCheck_DEPRECATED(DenseHashSet<TypePackId>&, TypePackId, TypePackId)`.

use core::mem::zeroed;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::occurs_check_result::OccursCheckResult,
  functions::{
    follow_type_pack::follow_type_pack_id, get_mutable_type_pack::get_mutable_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::{
    free_type_pack::FreeTypePack, recursion_limiter::RecursionLimiter, type_pack::TypePack,
    unifier_2::Unifier2,
  },
  type_aliases::{error_type_pack::ErrorTypePack, type_pack_id::TypePackId},
};
impl Unifier2 {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn occurs_check_deprecated(
    &mut self,
    seen: &mut DenseHashSet<TypePackId>,
    needle: TypePackId,
    mut haystack: TypePackId,
  ) -> OccursCheckResult {
    let needle = unsafe { follow_type_pack_id(needle) };
    haystack = unsafe { follow_type_pack_id(haystack) };

    if seen.find(&haystack).is_some() {
      return OccursCheckResult::Pass;
    }

    seen.insert(haystack);

    if get_mutable_type_pack_id::<ErrorTypePack>(needle).is_some() {
      return OccursCheckResult::Pass;
    }

    if get_mutable_type_pack_id::<FreeTypePack>(needle).is_none() {
      unsafe { (*self.ice.as_ptr()).ice_string("Expected needle pack to be free") };
    }

    let mut _ra = RecursionLimiter {
      base: unsafe { zeroed() },
      native_stack_guard: unsafe { zeroed() },
    };
    _ra.recursion_limiter_recursion_limiter(
      "Unifier2::occursCheck",
      &mut self.recursion_count,
      self.recursion_limit,
    );

    while get_mutable_type_pack_id::<ErrorTypePack>(haystack).is_none() {
      if needle == haystack {
        return OccursCheckResult::Fail;
      }

      // C++: if (auto a = get<TypePack>(haystack); a && a->tail)
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
}
