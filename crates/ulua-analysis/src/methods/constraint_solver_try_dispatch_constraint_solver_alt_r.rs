use crate::records::{
  constraint::Constraint, constraint_solver::ConstraintSolver,
  equality_constraint::EqualityConstraint,
};

impl ConstraintSolver {
  pub fn try_dispatch_equality_constraint_not_null_constraint(
    &mut self,
    c: &EqualityConstraint,
    constraint: *const Constraint,
  ) -> bool {
    self.constraint_solver_unify(constraint, c.result_type, c.assignment_type);
    self.constraint_solver_unify(constraint, c.assignment_type, c.result_type);
    true
  }
}
