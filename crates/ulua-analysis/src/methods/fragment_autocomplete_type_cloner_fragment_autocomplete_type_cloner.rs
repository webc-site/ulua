//! Source: `Analysis/src/Clone.cpp:478-491`
//! `FragmentAutocompleteTypeCloner::FragmentAutocompleteTypeCloner(...)`.

use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    builtin_types::BuiltinTypes,
    fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
    scope::Scope,
    type_arena::TypeArena,
    type_cloner::{SeenTypePacks, SeenTypes, TypeCloner},
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl FragmentAutocompleteTypeCloner {
  pub fn new(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    types: *mut SeenTypes,
    packs: *mut SeenTypePacks,
    force_ty: TypeId,
    force_tp: TypePackId,
    replacement_for_null_scope: *mut Scope,
  ) -> Self {
    LUAU_ASSERT!(!replacement_for_null_scope.is_null());
    Self {
      // `TypeCloner(arena, builtinTypes, types, packs, forceTy, forceTp)`.
      // The override state (Clone.cpp:493-518, 541-544) is carried on the
      // base so the shared clone machinery applies it to the whole subgraph.
      base: TypeCloner {
        arena,
        builtin_types,
        queue: Vec::new(),
        types,
        packs,
        force_ty,
        force_tp,
        steps: 0,
        replacement_for_null_scope,
        skip_lazy_type_clone: true,
      },
      replacement_for_null_scope,
    }
  }
}
