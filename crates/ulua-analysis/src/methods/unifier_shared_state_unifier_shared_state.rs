use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  internal_error_reporter::InternalErrorReporter, unifier_counters::UnifierCounters,
  unifier_shared_state::UnifierSharedState,
};
impl UnifierSharedState {
  pub fn new(ice_handler: *mut InternalErrorReporter) -> Self {
    Self {
      ice_handler,
      skip_cache_for_type: DenseHashMap::default(),
      cached_unify: DenseHashSet::default(),
      cached_unify_error: DenseHashMap::default(),
      temp_seen_ty: DenseHashSet::default(),
      temp_seen_tp: DenseHashSet::default(),
      counters: UnifierCounters::default(),
      reentrant_type_reduction: false,
    }
  }
}
