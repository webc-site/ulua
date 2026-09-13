use ulua_common::FFlag;

use crate::{
  functions::follow_type::follow_type_id,
  records::constraint_solver::ConstraintSolver,
  type_aliases::{constraint_vertex::ConstraintVertex, type_id::TypeId},
};
impl ConstraintSolver {
  pub fn has_unresolved_constraints(&mut self, ty: TypeId) -> bool {
    if FFlag::LuauConstraintGraph.get() {
      let ty = follow_type_id(ty);
      unsafe { (*self.cgraph).has_unsolved_dependencies(ConstraintVertex::V0(ty)) }
    } else {
      let ty = follow_type_id(ty);
      if let Some(set) = self.deprecated_type_to_constraint_set.get(&ty) {
        !set.is_empty()
      } else {
        false
      }
    }
  }
}
