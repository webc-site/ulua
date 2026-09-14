use ulua_common::FFlag;

use crate::{
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
impl ConstraintSolver {
  pub fn block_not_null_constraint_not_null_constraint(
    &mut self,
    target: *const Constraint,
    constraint: *const Constraint,
  ) {
    let new_block = if FFlag::LuauConstraintGraph.get() {
      unsafe {
        (*self.cgraph).add_dependency_of_constraint_constraint(
          &mut *(target as *mut Constraint),
          &mut *(constraint as *mut Constraint),
        )
      }
    } else {
      self.deprecate_d_block(BlockedConstraintId::V2(target), constraint)
    };

    if new_block && let Some(logger) = unsafe { self.logger.as_mut() } {
      logger.push_block_not_null_constraint_not_null_constraint(constraint, target);
    }
  }
}
