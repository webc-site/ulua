use core::mem::take;

use crate::records::{constraint::Constraint, dcr_logger::DcrLogger, scope::Scope};
impl DcrLogger {
  pub fn capture_initial_solver_state(
    &mut self,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) {
    let mut initial_state = take(&mut self.solve_log.initial_state);
    self.capture_boundary_state(&mut initial_state, root_scope, unsolved_constraints);
    self.solve_log.initial_state = initial_state;
  }
}
