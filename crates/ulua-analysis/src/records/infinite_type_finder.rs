use core::ptr::NonNull;

use crate::records::{
  constraint_solver::ConstraintSolver, instantiation_signature::InstantiationSignature,
  iterative_type_visitor::IterativeTypeVisitor, scope::Scope,
};

#[derive(Debug, Clone)]
pub struct InfiniteTypeFinder {
  pub base: IterativeTypeVisitor,
  pub solver: *mut ConstraintSolver,
  pub signature: InstantiationSignature,
  pub scope: NonNull<Scope>,
  pub found_infinite_type: bool,
}
