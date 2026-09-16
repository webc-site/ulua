//! @interface-stub
use core::ptr::null;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  internal_error_reporter::InternalErrorReporter, unifier_counters::UnifierCounters,
  unifier_shared_state::UnifierSharedState,
};
impl UnifierSharedState {
  pub fn new(ice_handler: *mut InternalErrorReporter) -> Self {
    Self {
      ice_handler,
      skip_cache_for_type: DenseHashMap::new(null()),
      cached_unify: DenseHashSet::new((null(), null())),
      cached_unify_error: DenseHashMap::new((null(), null())),
      temp_seen_ty: DenseHashSet::new(null()),
      temp_seen_tp: DenseHashSet::new(null()),
      counters: UnifierCounters::default(),
      reentrant_type_reduction: false,
    }
  }
}
