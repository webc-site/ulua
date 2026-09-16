use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::hash_blocked_constraint_id::HashBlockedConstraintId,
  type_aliases::constraint_vertex::ConstraintVertex,
};

#[derive(Debug, Clone)]
pub struct ConstraintList {
  pub(crate) present: DenseHashMap<ConstraintVertex, bool, HashBlockedConstraintId>,
  pub(crate) order: Vec<ConstraintVertex>,
  pub(crate) entries: usize,
}
