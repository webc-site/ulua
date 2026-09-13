//! @interface-stub
use ulua_ast::records::location::Location;

use crate::{
  functions::{follow_type::follow, get_type_alt_j::get_type_id, simplify_union::simplify_union},
  records::{
    constraint_generator::ConstraintGenerator, never_type::NeverType, scope::Scope,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `_scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn make_union_scope_ptr_location_type_id_type_id(
    &mut self,
    _scope: *mut Scope,
    _location: Location,
    lhs: TypeId,
    rhs: TypeId,
  ) -> TypeId {
    // 对照 C++ makeUnion：`if (get<NeverType>(follow(lhs))) return rhs;`
    if get_type_id::<NeverType>(follow(lhs)).is_some() {
      return rhs;
    }

    if get_type_id::<NeverType>(follow(rhs)).is_some() {
      return lhs;
    }

    let result = simplify_union(self.builtin_types, self.arena, lhs, rhs).result;

    if get_type_id::<UnionType>(follow(result)).is_some() {
      self.unions_to_simplify.push(result);
    }

    result
  }
}
