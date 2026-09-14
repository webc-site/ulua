use ulua_common::FFlag;

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, constraint_vertex::ConstraintVertex,
    type_pack_id::TypePackId,
  },
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `constraint` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn block_type_pack_id_not_null_constraint(
    &mut self,
    target: TypePackId,
    constraint: *const Constraint,
  ) -> bool {
    let target = unsafe { follow_type_pack_id(target) };
    let new_block = if FFlag::LuauConstraintGraph.get() {
      unsafe {
        (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
          ConstraintVertex::V1(target),
          ConstraintVertex::V2(constraint),
        )
      }
    } else {
      self.deprecate_d_block(BlockedConstraintId::V1(target), constraint)
    };

    if new_block && let Some(logger) = unsafe { self.logger.as_mut() } {
      logger.push_block_not_null_constraint_type_pack_id(constraint, target);
    }
    false
  }
}
