use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use std::collections::HashMap;

use crate::{
  functions::clone_clone_alt_b::with_clone_maps,
  records::{clone_state::CloneState, type_arena::TypeArena, type_cloner::TypeCloner},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn clone(
  tp: TypePackId,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
) -> TypePackId {
  if unsafe { (*tp).persistent } {
    return tp;
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
      cloner.clone_type_pack_id(tp)
    },
  )
}
