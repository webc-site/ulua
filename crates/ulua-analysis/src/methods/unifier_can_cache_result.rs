use std::collections::HashSet;

use crate::{
  records::{skip_cache_for_type::SkipCacheForType, unifier::Unifier},
  type_aliases::type_id::TypeId,
};
impl Unifier {
  pub fn unifier_can_cache_result(&mut self, sub_ty: TypeId, super_ty: TypeId) -> bool {
    let shared_state = unsafe { &*self.shared_state };

    if let Some(super_ty_info) = shared_state.skip_cache_for_type.find(&super_ty)
      && *super_ty_info
    {
      return false;
    }

    if let Some(sub_ty_info) = shared_state.skip_cache_for_type.find(&sub_ty)
      && *sub_ty_info
    {
      return false;
    }

    let skip_cache_for = |ty: TypeId| -> bool {
      let mut visitor = SkipCacheForType::skip_cache_for_type_skip_cache_for_type(
        &shared_state.skip_cache_for_type,
        self.types,
      );
      // C++ `visitor.traverse(ty)` — dispatch to the per-variant visit
      // overrides and recurse into composite types, so any nested mutable
      // element (unsealed/free table, free/bound/generic/blocked pack,
      // etc.) flips `result` and makes the unification uncacheable.
      let mut seen_types = HashSet::new();
      let mut seen_packs = HashSet::new();
      unsafe { visitor.traverse_skip_cache(ty, &mut seen_types, &mut seen_packs) };

      let mut_shared_state = unsafe { &mut *self.shared_state };
      mut_shared_state
        .skip_cache_for_type
        .try_insert(ty, visitor.result);
      visitor.result
    };

    if shared_state.skip_cache_for_type.find(&super_ty).is_none() && skip_cache_for(super_ty) {
      return false;
    }

    if shared_state.skip_cache_for_type.find(&sub_ty).is_none() && skip_cache_for(sub_ty) {
      return false;
    }

    true
  }
}
