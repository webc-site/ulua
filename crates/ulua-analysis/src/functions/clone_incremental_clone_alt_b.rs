//! Source: `Analysis/src/Clone.cpp:660-675`
//! `TypeId cloneIncremental(TypeId typeId, TypeArena& dest, CloneState& cloneState, Scope* freshScopeForFreeTypes)`.

use core::ptr::null;
use std::collections::HashMap;

use crate::{
  functions::clone_clone_alt_b::with_clone_maps,
  records::{
    clone_state::CloneState, fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
    scope::Scope, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn clone_incremental(
  type_id: TypeId,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  fresh_scope_for_free_types: *mut Scope,
) -> TypeId {
  if unsafe { (*type_id).persistent } {
    return type_id;
  }

  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = FragmentAutocompleteTypeCloner::new(
        dest as *mut TypeArena,
        builtin_types,
        tys as *mut HashMap<TypeId, TypeId>,
        tps as *mut HashMap<TypePackId, TypePackId>,
        null(),
        null(),
        fresh_scope_for_free_types,
      );
      cloner.base.clone_type_id(type_id)
    },
  )
}
