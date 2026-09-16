use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::constraint_solver::ConstraintSolver,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintSolver {
  pub fn deprecate_d_unblock_(&mut self, progressed: BlockedConstraintId) {
    LUAU_ASSERT!(!FFlag::LuauConstraintGraph.get());
    if let Some(blocked_constraints) = self.deprecated_blocked.remove(&progressed) {
      for unblocked_constraint in blocked_constraints.iter() {
        let count = self
          .deprecated_blocked_constraints
          .get_mut(unblocked_constraint)
          .unwrap();
        LUAU_ASSERT!(*count > 0);
        *count -= 1;
      }
    }
  }
}
