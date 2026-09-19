use crate::records::constraint_solver::ConstraintSolver;

impl ConstraintSolver {
  pub fn is_done(&self) -> bool {
    self.unsolved_constraints.is_empty()
  }
}
