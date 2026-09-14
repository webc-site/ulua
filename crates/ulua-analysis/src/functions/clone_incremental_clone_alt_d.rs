//! Source: `Analysis/src/Clone.cpp:712-732`
//! `Binding cloneIncremental(const Binding& binding, TypeArena& dest, CloneState& cloneState, Scope* freshScopeForFreeTypes)`.

use core::ptr::null;
use std::collections::HashMap;

use crate::{
  functions::clone_clone_alt_b::with_clone_maps,
  records::{
    binding::Binding, clone_state::CloneState,
    fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner, scope::Scope,
    type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn clone_incremental(
  binding: &Binding,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  fresh_scope_for_free_types: *mut Scope,
) -> Binding {
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

      Binding {
        deprecated: binding.deprecated,
        deprecated_suggestion: binding.deprecated_suggestion.clone(),
        documentation_symbol: binding.documentation_symbol.clone(),
        location: binding.location,
        // Clone.cpp:729: `binding.typeId->persistent ? binding.typeId : cloner.clone(binding.typeId)`.
        type_id: if unsafe { (*binding.type_id).persistent } {
          binding.type_id
        } else {
          cloner.base.clone_type_id(binding.type_id)
        },
      }
    },
  )
}
