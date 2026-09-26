use alloc::string::String;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  constraint::Constraint, constraint_snapshot::ConstraintSnapshot, scope_snapshot::ScopeSnapshot,
};
#[derive(Debug, Clone)]
pub struct ConstraintStepSnapshot {
  pub(crate) current_constraint: *const Constraint,
  pub(crate) forced: bool,
  pub(crate) unsolved_constraints: DenseHashMap<*const Constraint, ConstraintSnapshot>,
  pub(crate) root_scope: ScopeSnapshot,
  pub(crate) type_strings: DenseHashMap<*const (), String>,
}
