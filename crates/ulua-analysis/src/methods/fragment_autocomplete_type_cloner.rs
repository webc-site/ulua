//! `fragment_autocomplete_type_cloner` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::Handle,
    builtin_types::BuiltinTypes,
    fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner,
    scope::Scope,
    type_arena::TypeArena,
    type_cloner::{SeenTypePacks, SeenTypes, TypeCloner},
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// Source: `Analysis/src/Clone.cpp:478-491`
// `FragmentAutocompleteTypeCloner::FragmentAutocompleteTypeCloner(...)`.

impl FragmentAutocompleteTypeCloner {
  pub fn new(
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
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

impl FragmentAutocompleteTypeCloner {
  pub fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
    self.base.shallow_clone_type_id(ty)
  }

  pub fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    self.base.shallow_clone_type_pack_id(tp)
  }
}
