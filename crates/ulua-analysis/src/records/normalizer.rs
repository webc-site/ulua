use alloc::{collections::BTreeMap, sync::Arc};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, normalized_type::NormalizedType, type_arena::TypeArena,
    type_id_pair_hash::TypeIdPairHash, type_ids::TypeIds, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct Normalizer {
  pub(crate) cached_normals: BTreeMap<TypeId, Arc<NormalizedType>>,
  pub(crate) cached_intersections: BTreeMap<*const TypeIds, TypeId>,
  pub(crate) cached_unions: BTreeMap<*const TypeIds, TypeId>,
  pub(crate) cached_type_ids: BTreeMap<*const TypeIds, Box<TypeIds>>,
  pub(crate) cached_is_inhabited: DenseHashMap<TypeId, bool>,
  pub(crate) cached_is_inhabited_intersection: DenseHashMap<(TypeId, TypeId), bool, TypeIdPairHash>,
  pub(crate) fuel: Option<i32>,
  pub(crate) arena: *mut TypeArena,
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) shared_state: *mut UnifierSharedState,
  pub(crate) cache_inhabitance: bool,
  pub(crate) solver_mode: SolverMode,
}
