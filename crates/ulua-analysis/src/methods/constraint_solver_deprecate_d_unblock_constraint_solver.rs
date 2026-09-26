use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{
    blocked_constraint_registry::register_constraint, constraint::Constraint,
    constraint_solver::ConstraintSolver,
  },
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintSolver {
  pub fn deprecate_d_unblock_(&mut self, progressed: BlockedConstraintId) {
    LUAU_ASSERT!(!fflag::LuauConstraintGraph.get());
    if let Some(blocked_constraints) = self.deprecated_blocked.remove(&progressed) {
      for unblocked_constraint in blocked_constraints.iter() {
        let count = self
          .deprecated_blocked_constraints
          .get_mut(unblocked_constraint)
          // Safety: 与 deprecated_blocked 成对登记的键，constraints 表必命中。
          .expect("blocked_constraints 与 constraints 成对登记，键必命中");
        LUAU_ASSERT!(*count > 0);
        *count -= 1;
      }
    }
  }

  pub fn constraint_solver_deprecate_d_unblock(&mut self, progressed: *const Constraint) {
    if let Some(logger) = unsafe { self.logger.as_mut() } {
      logger.pop_block_not_null_constraint(progressed);
    }
    self.deprecate_d_unblock_(BlockedConstraintId::V2(register_constraint(progressed)));
  }
}
