use alloc::{string::String, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  constraint_solver::ConstraintSolver, infinite_type_finder::InfiniteTypeFinder,
  instantiation_signature::InstantiationSignature, iterative_type_visitor::IterativeTypeVisitor,
  scope::Scope,
};
impl InfiniteTypeFinder {
  pub fn infinite_type_finder_infinite_type_finder(
    solver: *mut ConstraintSolver,
    signature: &InstantiationSignature,
    scope: NonNull<Scope>,
  ) -> Self {
    let mut visitor = InfiniteTypeFinder {
      base: IterativeTypeVisitor {
        seen: DenseHashSet::new(null_mut()),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("InfiniteTypeFinder"),
        skip_bound_types: true,
        visit_once: true,
      },
      solver,
      signature: signature.clone(),
      scope,
      found_infinite_type: false,
    };
    visitor
      .base
      .iterative_type_visitor_string_bool_bool("InfiniteTypeFinder", true, true);
    visitor
  }
}
