use alloc::vec::Vec;
use core::ptr::{NonNull, null, null_mut};

use ulua_common::{
  dfint,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  records::{
    builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter, scope::Scope,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits, type_pair_hash::TypePairHash,
    unifier_2::Unifier2,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Unifier2 {
  pub fn unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
    arena: NonNull<TypeArena>,
    builtin_types: NonNull<BuiltinTypes>,
    scope: NonNull<Scope>,
    ice: NonNull<InternalErrorReporter>,
  ) -> Self {
    Self::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
            arena,
            builtin_types,
            scope,
            ice,
            null_mut(),
        )
  }

  pub fn unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
    _arena: NonNull<TypeArena>,
    _builtin_types: NonNull<BuiltinTypes>,
    _scope: NonNull<Scope>,
    _ice: NonNull<InternalErrorReporter>,
    _uninhabited_type_functions: *mut DenseHashSet<*const ()>,
  ) -> Self {
    Unifier2 {
      arena: _arena,
      builtin_types: _builtin_types,
      scope: _scope,
      _ice,
      limits: TypeCheckLimits::default(),
      seen_type_pairings: DenseHashSet::<(TypeId, TypeId), TypePairHash>::new((null(), null())),
      seen_type_pack_pairings: DenseHashSet::<(TypePackId, TypePackId), TypePairHash>::new((
        null(),
        null(),
      )),
      expanded_free_types: DenseHashMap::default(),
      generic_substitutions: DenseHashMap::default(),
      generic_pack_substitutions: DenseHashMap::default(),
      new_fresh_types: Vec::new(),
      new_fresh_type_packs: Vec::new(),
      iteration_count: 0,
      recursion_count: 0,
      recursion_limit: dfint::LuauUnifierRecursionLimit.get(),
      incomplete_subtypes: Vec::new(),
      uninhabited_type_functions: _uninhabited_type_functions,
    }
  }
}
