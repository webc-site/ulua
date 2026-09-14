use ulua_ast::records::location::Location;

use crate::{
  functions::simplify_intersection_simplify::simplify_intersection,
  records::{constraint_solver::ConstraintSolver, scope::Scope},
  type_aliases::type_id::TypeId,
};

impl ConstraintSolver {
  pub fn simplify_intersection_not_null_scope_location_type_id_type_id(
    &mut self,
    _scope: *mut Scope,
    _location: Location,
    left: TypeId,
    right: TypeId,
  ) -> TypeId {
    simplify_intersection(self.builtin_types, self.arena, left, right).result
  }
}
