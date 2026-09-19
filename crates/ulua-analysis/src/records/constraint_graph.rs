use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    constraint_list::ConstraintList, hash_blocked_constraint_id::HashBlockedConstraintId,
    pinned_storage::PinnedStorage,
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

/// C++ `ConstraintGraph::dependencies` 的映射类型（Analysis/include/Luau/ConstraintGraph.h）
pub type ConstraintMap =
  DenseHashMap<BlockedConstraintId, *mut ConstraintList, HashBlockedConstraintId>;

#[derive(Debug, Clone)]
pub struct ConstraintGraph {
  pub(crate) dependencies: ConstraintMap,
  pub(crate) reverse_dependencies: ConstraintMap,
  pub(crate) constraint_lists: PinnedStorage<ConstraintList>,
}
