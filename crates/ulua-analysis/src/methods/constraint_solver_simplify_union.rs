use ulua_ast::records::location::Location;

use crate::{
  functions::simplify_union::simplify_union,
  records::{constraint_solver::ConstraintSolver, scope::Scope},
  type_aliases::type_id::TypeId,
};

impl ConstraintSolver {
  pub(crate) fn simplify_union(
    &mut self,
    _scope: *mut Scope,
    _location: Location,
    left: TypeId,
    right: TypeId,
  ) -> TypeId {
    simplify_union(self.builtin_types, self.arena, left, right).result
  }
}
