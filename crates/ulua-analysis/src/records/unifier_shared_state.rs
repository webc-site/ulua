use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    internal_error_reporter::InternalErrorReporter, type_id_pair_hash::TypeIdPairHash,
    unifier_counters::UnifierCounters,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug)]
pub struct UnifierSharedState {
  pub(crate) ice_handler: *mut InternalErrorReporter,
  pub(crate) skip_cache_for_type: DenseHashMap<TypeId, bool>,
  pub(crate) cached_unify: DenseHashSet<(TypeId, TypeId), TypeIdPairHash>,
  pub(crate) cached_unify_error: DenseHashMap<(TypeId, TypeId), TypeErrorData, TypeIdPairHash>,
  pub(crate) temp_seen_ty: DenseHashSet<TypeId>,
  pub(crate) temp_seen_tp: DenseHashSet<TypePackId>,
  pub(crate) counters: UnifierCounters,
  pub(crate) reentrant_type_reduction: bool,
}
