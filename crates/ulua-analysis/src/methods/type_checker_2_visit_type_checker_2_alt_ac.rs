use core::mem::take;

use ulua_ast::records::ast_expr_constant_bool::AstExprConstantBool;

use crate::records::{type_checker_2::TypeChecker2, type_mismatch::TypeMismatch};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_constant_bool(&mut self, expr: *mut AstExprConstantBool) {
    // booleans use specialized inference logic for singleton type_arguments,
    // which can lead to real type errors here.
    unsafe {
      let builtin_types = &*self.builtin_types;
      let best_type = if (*expr).value {
        builtin_types.true_type
      } else {
        builtin_types.false_type
      };
      let inferred_type = self.lookup_type(&(*expr).base);
      let scope = self.find_innermost_scope((*expr).base.base.location);

      let mut r = (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
        best_type,
        inferred_type,
        scope,
      );
      if !r.is_error_suppressing {
        if !r.is_subtype {
          self.report_error_type_error_data_location(
            TypeMismatch::from_wanted_given(inferred_type, best_type).into(),
            &(*expr).base.base.location,
          );
        }
        for e in &mut r.errors {
          e.location = (*expr).base.base.location;
        }
        self.report_errors(take(&mut r.errors));
      }
    }
  }
}
