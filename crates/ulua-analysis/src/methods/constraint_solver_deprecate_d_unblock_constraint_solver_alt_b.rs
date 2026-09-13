use crate::{
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
impl ConstraintSolver {
  pub fn constraint_solver_deprecate_d_unblock(&mut self, progressed: *const Constraint) {
    if let Some(logger) = unsafe { self.logger.as_mut() } {
      logger.pop_block_not_null_constraint(progressed);
    }
    self.deprecate_d_unblock_(BlockedConstraintId::V2(progressed));
  }
}
