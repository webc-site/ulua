use alloc::string::String;
use core::{ffi::c_void, ptr::null};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  constraint::Constraint, constraint_snapshot::ConstraintSnapshot, scope_snapshot::ScopeSnapshot,
};
#[derive(Debug, Clone)]
pub struct BoundarySnapshot {
  pub(crate) unsolved_constraints: DenseHashMap<*const Constraint, ConstraintSnapshot>,
  pub(crate) root_scope: ScopeSnapshot,
  pub(crate) type_strings: DenseHashMap<*const c_void, String>,
}

impl Default for BoundarySnapshot {
  fn default() -> Self {
    Self {
      unsolved_constraints: DenseHashMap::new(null()),
      root_scope: ScopeSnapshot::default(),
      type_strings: DenseHashMap::new(null()),
    }
  }
}
