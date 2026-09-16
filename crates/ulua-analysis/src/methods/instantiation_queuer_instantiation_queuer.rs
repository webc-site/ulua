use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  records::{
    constraint_solver::ConstraintSolver, instantiation_queuer::InstantiationQueuer,
    iterative_type_visitor::IterativeTypeVisitor, scope::Scope,
  },
  type_aliases::seen_set_iterative_type_visitor::SeenSet,
};
impl InstantiationQueuer {
  pub fn new(scope: NonNull<Scope>, location: &Location, solver: *mut ConstraintSolver) -> Self {
    let mut visitor = InstantiationQueuer {
      base: IterativeTypeVisitor {
        seen: SeenSet::default(),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("InstantiationQueuer"),
        skip_bound_types: true,
        visit_once: true,
      },
      solver,
      scope,
      location: *location,
    };
    visitor
      .base
      .iterative_type_visitor_string_bool_bool("InstantiationQueuer", true, true);
    visitor
  }
}
