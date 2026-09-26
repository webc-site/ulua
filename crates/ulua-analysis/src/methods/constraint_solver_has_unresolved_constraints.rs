use ulua_common::fflag;

use crate::{
  functions::follow_type,
  records::constraint_solver::ConstraintSolver,
  type_aliases::{blocked_constraint_id::BlockedConstraintId, type_id::TypeId},
};
impl ConstraintSolver {
  pub fn has_unresolved_constraints(&mut self, ty: TypeId) -> bool {
    if fflag::LuauConstraintGraph.get() {
      let ty = follow_type::follow(ty);
      unsafe { (*self.cgraph).has_unsolved_dependencies(BlockedConstraintId::V0(ty)) }
    } else {
      let ty = follow_type::follow(ty);
      if let Some(set) = self.deprecated_type_to_constraint_set.get(&ty) {
        !set.is_empty()
      } else {
        false
      }
    }
  }
}
