use ulua_ast::records::location::Location;

use crate::{
  functions::simplify_intersection_simplify::simplify_intersection,
  records::{constraint_solver::ConstraintSolver, scope::Scope, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl ConstraintSolver {
  pub fn simplify_intersection_not_null_scope_location_type_ids(
    &mut self,
    _scope: *mut Scope,
    _location: Location,
    parts: TypeIds,
  ) -> TypeId {
    let left = parts.front();
    let mut parts = parts;
    parts.erase_type_id(left);
    let right = parts.front();

    simplify_intersection(self.builtin_types, self.arena, left, right).result
  }
}
