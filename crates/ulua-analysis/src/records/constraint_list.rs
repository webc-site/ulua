use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::hash_blocked_constraint_id::HashBlockedConstraintId,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

#[derive(Debug, Clone)]
pub struct ConstraintList {
  pub(crate) present: DenseHashMap<BlockedConstraintId, bool, HashBlockedConstraintId>,
  pub(crate) order: Vec<BlockedConstraintId>,
  pub(crate) entries: usize,
}
