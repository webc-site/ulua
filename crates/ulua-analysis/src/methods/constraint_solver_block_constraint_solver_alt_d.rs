use ulua_common::fflag;

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::{blocked_constraint_id::BlockedConstraintId, type_pack_id::TypePackId},
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
    let new_block = if fflag::LuauConstraintGraph.get() {
      unsafe {
        (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
          BlockedConstraintId::V1(target),
          BlockedConstraintId::V2(constraint),
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
