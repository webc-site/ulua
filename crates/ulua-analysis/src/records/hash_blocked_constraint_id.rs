use ulua_common::records::dense_hash_table::DenseHasher;

use crate::type_aliases::blocked_constraint_id::BlockedConstraintId;

// C++ (ConstraintGraph.h:21-24): a hash functor over BlockedConstraintId;
// `operator()` is implemented in ConstraintGraph.cpp as its own method node.
#[derive(Debug, Clone, Copy, Default)]
pub struct HashBlockedConstraintId;

// Bridge the C++ `Hash` template parameter (`HashBlockedConstraintId`) to the
// `DenseHasher` trait the `DenseHashMap` port is generic over. The hash itself
// lives in the `operator_call` method node.
impl DenseHasher<BlockedConstraintId> for HashBlockedConstraintId {
  fn hash(&self, key: &BlockedConstraintId) -> usize {
    self.operator_call(key)
  }
}
