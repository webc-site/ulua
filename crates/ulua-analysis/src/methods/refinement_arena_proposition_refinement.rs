use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    proposition_refinement::Proposition, refinement_arena_refinement::RefinementArena,
    refinement_key::RefinementKey,
  },
  type_aliases::{
    refinement_id_refinement::RefinementId, refinement_refinement::Refinement, type_id::TypeId,
  },
};
impl RefinementArena {
  /// §2：cpp `if (!key) return nullptr` 收口为 `Option`——空 key 返回 `None`，
  /// 本文件不再制造 `null_mut()` 哨兵。
  pub fn proposition_refinement_key_type_id(
    &mut self,
    key: *const RefinementKey,
    discriminant_ty: TypeId,
  ) -> Option<RefinementId> {
    if key.is_null() {
      return None;
    }

    let refinement_ptr = self
      .allocator
      .allocate(Refinement::Proposition(Proposition {
        key,
        discriminant_ty,
        implicit_from_call: false,
      }));

    LUAU_ASSERT!(!refinement_ptr.is_null());
    Some(refinement_ptr)
  }
}
