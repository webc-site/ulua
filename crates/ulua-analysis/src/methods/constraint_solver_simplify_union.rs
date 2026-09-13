use ulua_ast::records::location::Location;

use crate::{
  functions::simplify_union::simplify_union,
  records::{constraint_solver::ConstraintSolver, scope::Scope},
  type_aliases::type_id::TypeId,
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn simplify_union(
    &mut self,
    scope: *mut Scope,
    location: Location,
    left: TypeId,
    right: TypeId,
  ) -> TypeId {
    let builtin_types = self.builtin_types;
    let arena = self.arena;
    let _ = scope;
    let _ = location;
    simplify_union(builtin_types, arena, left, right).result
  }
}
