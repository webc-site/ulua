//! Source: `Analysis/src/Clone.cpp:677-710`
//! `TypeFun cloneIncremental(const TypeFun& typeFun, TypeArena& dest, CloneState& cloneState, Scope* freshScopeForFreeTypes)`.

use core::ptr::null;
use std::collections::HashMap;

use crate::{
  functions::clone_clone_alt_b::with_clone_maps,
  records::{
    clone_state::CloneState, fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
    scope::Scope, type_arena::TypeArena, type_fun::TypeFun,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn clone_incremental(
  type_fun: &TypeFun,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  fresh_scope_for_free_types: *mut Scope,
) -> TypeFun {
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

      let mut copy = type_fun.clone();

      for param in copy.type_params.iter_mut() {
        param.ty = cloner.base.clone_type_id(param.ty);

        if let Some(default_value) = param.default_value {
          param.default_value = Some(cloner.base.clone_type_id(default_value));
        }
      }

      for param in copy.type_pack_params.iter_mut() {
        param.tp = cloner.base.clone_type_pack_id(param.tp);

        if let Some(default_value) = param.default_value {
          param.default_value = Some(cloner.base.clone_type_pack_id(default_value));
        }
      }

      copy.r#type = cloner.base.clone_type_id(copy.r#type);

      copy
    },
  )
}
