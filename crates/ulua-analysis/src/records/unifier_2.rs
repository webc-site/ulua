use alloc::vec::Vec;
use core::{ffi::c_void, ptr::NonNull};

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter, scope::Scope,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits, type_pair_hash::TypePairHash,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug)]
pub struct Unifier2 {
  pub(crate) arena: NonNull<TypeArena>,
  pub(crate) builtin_types: NonNull<BuiltinTypes>,
  pub(crate) scope: NonNull<Scope>,
  pub(crate) ice: NonNull<InternalErrorReporter>,
  pub limits: TypeCheckLimits,
  pub(crate) seen_type_pairings: DenseHashSet<(TypeId, TypeId), TypePairHash>,
  pub(crate) seen_type_pack_pairings: DenseHashSet<(TypePackId, TypePackId), TypePairHash>,
  pub(crate) expanded_free_types: DenseHashMap<TypeId, Vec<TypeId>>,
  pub(crate) generic_substitutions: DenseHashMap<TypeId, TypeId>,
  pub(crate) generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
  pub(crate) new_fresh_types: Vec<TypeId>,
  pub(crate) new_fresh_type_packs: Vec<TypePackId>,
  pub(crate) iteration_count: i32,
  pub(crate) recursion_count: i32,
  pub(crate) recursion_limit: i32,
  pub(crate) incomplete_subtypes: Vec<ConstraintV>,
  pub(crate) uninhabited_type_functions: *mut DenseHashSet<*const c_void>,
}
