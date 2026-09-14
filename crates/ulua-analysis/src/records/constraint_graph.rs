use core::ptr::NonNull;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    builtin_types::BuiltinTypes, constraint_list::ConstraintList,
    hash_blocked_constraint_id::HashBlockedConstraintId, pinned_storage::PinnedStorage,
  },
  type_aliases::constraint_vertex::ConstraintVertex,
};

/// C++ `ConstraintGraph::dependencies` 的映射类型（Analysis/include/Luau/ConstraintGraph.h）
pub type ConstraintMap =
  DenseHashMap<ConstraintVertex, *mut ConstraintList, HashBlockedConstraintId>;

#[derive(Debug, Clone)]
pub struct ConstraintGraph {
  pub(crate) builtin_types: NonNull<BuiltinTypes>,
  pub(crate) dependencies: ConstraintMap,
  pub(crate) reverse_dependencies: ConstraintMap,
  pub(crate) constraint_lists: PinnedStorage<ConstraintList>,
}
