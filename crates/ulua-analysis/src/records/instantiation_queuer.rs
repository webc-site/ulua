use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::records::{
  constraint_solver::ConstraintSolver, iterative_type_visitor::IterativeTypeVisitor, scope::Scope,
};

#[derive(Debug, Clone)]
pub struct InstantiationQueuer {
  pub base: IterativeTypeVisitor,
  pub solver: *mut ConstraintSolver,
  pub scope: NonNull<Scope>,
  pub location: Location,
}
