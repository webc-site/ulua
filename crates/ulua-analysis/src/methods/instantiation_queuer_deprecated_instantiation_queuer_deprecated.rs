use alloc::string::String;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::records::{
  constraint_solver::ConstraintSolver,
  instantiation_queuer_deprecated::InstantiationQueuerDeprecated, scope::Scope,
  type_once_visitor::TypeOnceVisitor,
};

impl InstantiationQueuerDeprecated {
  pub fn instantiation_queuer_deprecated_instantiation_queuer_deprecated(
    scope: NonNull<Scope>,
    location: &Location,
    solver: *mut ConstraintSolver,
  ) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("InstantiationQueuer"), true),
      solver,
      scope,
      location: *location,
    }
  }
}
