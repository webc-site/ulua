use alloc::string::String;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  constraint::Constraint, constraint_snapshot::ConstraintSnapshot, scope_snapshot::ScopeSnapshot,
};
#[derive(Debug, Clone)]
pub struct GeneralizeStepSnapshot {
  pub(crate) before: String,
  pub(crate) after: String,
  pub(crate) unsolved_constraints: DenseHashMap<*const Constraint, ConstraintSnapshot>,
  pub(crate) root_scope: ScopeSnapshot,
  pub(crate) type_strings: DenseHashMap<*const (), String>,
}
