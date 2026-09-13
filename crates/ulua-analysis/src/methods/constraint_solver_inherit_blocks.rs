use alloc::vec::Vec;

use crate::{
  FFlag,
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::{blocked_constraint_id::BlockedConstraintId, constraint_vertex::ConstraintVertex},
};
impl ConstraintSolver {
  pub fn inherit_blocks(&mut self, source: *const Constraint, addition: *const Constraint) {
    if FFlag::LuauConstraintGraph.get() {
      unsafe {
        (*self.cgraph).inherit_blocks(ConstraintVertex::V2(source), ConstraintVertex::V2(addition))
      }
    } else {
      // Anything that is blocked on this constraint must also be blocked on our
      // synthesized constraints.
      let blocked_constraints: Vec<*const Constraint> = match self
        .deprecated_blocked
        .get(&BlockedConstraintId::V2(source))
      {
        Some(blocked_set) => blocked_set.iter().copied().collect(),
        None => Vec::new(),
      };
      for blocked_constraint in blocked_constraints {
        self.block_not_null_constraint_not_null_constraint(addition, blocked_constraint);
      }
    }
  }
}
