/// Bridge between `CloneState`'s `DenseHashMap`-backed seen-maps and the
/// `TypeCloner`'s `std::HashMap`-backed `SeenTypes`/`SeenTypePacks`. The C++ uses
/// a single `SeenTypes`/`SeenTypePacks` alias for both, so a faithful port has
/// the cloner read from and write back into the `CloneState`'s caches. This
/// seeds a local `HashMap` from the `DenseHashMap`, runs `body`, then writes any
/// newly-discovered entries back so cross-call deduplication is preserved.
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use std::collections::HashMap;

use crate::{
  records::{clone_state::CloneState, type_arena::TypeArena, type_cloner::TypeCloner},
  type_aliases::{
    seen_type_packs_clone::SeenTypePacks as DenseSeenTypePacks,
    seen_types_clone::SeenTypes as DenseSeenTypes, type_id::TypeId, type_pack_id::TypePackId,
  },
};
pub(crate) fn with_clone_maps<R>(
  seen_types: &mut DenseSeenTypes,
  seen_type_packs: &mut DenseSeenTypePacks,
  body: impl FnOnce(&mut HashMap<TypeId, TypeId>, &mut HashMap<TypePackId, TypePackId>) -> R,
) -> R {
  let mut tys: HashMap<TypeId, TypeId> = seen_types.iter().map(|(k, v)| (*k, *v)).collect();
  let mut tps: HashMap<TypePackId, TypePackId> =
    seen_type_packs.iter().map(|(k, v)| (*k, *v)).collect();

  let result = body(&mut tys, &mut tps);

  for (k, v) in tys.iter() {
    *seen_types.get_or_insert(*k) = *v;
  }
  for (k, v) in tps.iter() {
    *seen_type_packs.get_or_insert(*k) = *v;
  }

  result
}

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn clone(type_id: TypeId, dest: &mut TypeArena, clone_state: &mut CloneState) -> TypeId {
  if unsafe { (*type_id).persistent } {
    return type_id;
  }

  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = TypeCloner {
        arena: dest as *mut TypeArena,
        builtin_types,
        queue: Vec::new(),
        types: tys as *mut HashMap<TypeId, TypeId>,
        packs: tps as *mut HashMap<TypePackId, TypePackId>,
        force_ty: null(),
        force_tp: null(),
        steps: 0,
        replacement_for_null_scope: null_mut(),
        skip_lazy_type_clone: false,
      };
      cloner.clone_type_id(type_id)
    },
  )
}
