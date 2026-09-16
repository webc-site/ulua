use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
impl ConstraintSolver {
  pub fn deprecate_d_block(
    &mut self,
    target: BlockedConstraintId,
    constraint: *const Constraint,
  ) -> bool {
    let block_vec = self
      .deprecated_blocked
      .entry(target)
      .or_insert_with(|| DenseHashSet::new(null()));

    if block_vec.find(&constraint).is_some() {
      return false;
    }

    block_vec.insert(constraint);

    let count = self
      .deprecated_blocked_constraints
      .entry(constraint)
      .or_insert(0);
    *count += 1;

    true
  }
}
