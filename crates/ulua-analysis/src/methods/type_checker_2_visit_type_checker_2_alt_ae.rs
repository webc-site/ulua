use ulua_ast::records::ast_expr_constant_integer::AstExprConstantInteger;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_constant_integer(&mut self, expr: *mut AstExprConstantInteger) {
    #[cfg(any(debug_assertions, feature = "luau_assert"))]
    {
      unsafe {
        let builtin_types = &*self.builtin_types;
        let best_type = builtin_types.integer_type;
        let inferred_type = self.lookup_type(&(*expr).base);
        let scope = self.find_innermost_scope((*expr).base.base.location);

        let subtyping = &mut *self.subtyping;
        let r =
          subtyping.is_subtype_type_id_type_id_not_null_scope(best_type, inferred_type, scope);

        LUAU_ASSERT!(
          r.is_subtype
            || self
              .is_error_suppressing_location_type_id((*expr).base.base.location, inferred_type)
        );
      }
    }
    #[cfg(not(any(debug_assertions, feature = "luau_assert")))]
    let _ = expr;
  }
}
