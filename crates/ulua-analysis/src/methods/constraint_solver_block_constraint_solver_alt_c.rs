use ulua_common::FFlag;

use crate::{
  functions::follow_type::follow_type_id,
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, constraint_vertex::ConstraintVertex,
    type_id::TypeId,
  },
};
impl ConstraintSolver {
  pub fn block_type_id_not_null_constraint(
    &mut self,
    target: TypeId,
    constraint: *const Constraint,
  ) -> bool {
    let target = follow_type_id(target);
    let new_block = if FFlag::LuauConstraintGraph.get() {
      unsafe {
        (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
          ConstraintVertex::V0(target),
          ConstraintVertex::V2(constraint),
        )
      }
    } else {
      self.deprecate_d_block(BlockedConstraintId::V0(target), constraint)
    };

    if new_block && let Some(logger) = unsafe { self.logger.as_mut() } {
      logger.push_block_not_null_constraint_type_id(constraint, target);
    }
    false
  }
}
