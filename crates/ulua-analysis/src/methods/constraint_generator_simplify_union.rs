use ulua_ast::records::location::Location;

use crate::{
  functions::simplify_union::simplify_union,
  records::constraint_generator::ConstraintGenerator,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn simplify_union(
    &mut self,
    _scope: ScopePtr,
    _location: Location,
    left: TypeId,
    right: TypeId,
  ) -> TypeId {
    simplify_union(self.builtin_types, self.arena, left, right).result
  }
}
