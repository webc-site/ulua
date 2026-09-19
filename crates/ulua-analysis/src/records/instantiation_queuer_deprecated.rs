use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::records::{
  constraint_solver::ConstraintSolver, scope::Scope, type_once_visitor::TypeOnceVisitor,
};

#[derive(Debug, Clone)]
pub struct InstantiationQueuerDeprecated {
  pub base: TypeOnceVisitor,
  pub solver: *mut ConstraintSolver,
  pub scope: NonNull<Scope>,
  pub location: Location,
}
