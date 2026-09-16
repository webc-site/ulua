use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{records::scope::Scope, type_aliases::constraint_v::ConstraintV};

#[derive(Debug, Clone)]
pub struct Constraint {
  pub(crate) scope: *mut Scope,
  pub(crate) location: Location,
  pub(crate) c: ConstraintV,
  pub(crate) deprecated_dependencies: Vec<*mut Constraint>,
}
