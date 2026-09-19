use ulua_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

use crate::records::expected_type_visitor::ExpectedTypeVisitor;

impl ExpectedTypeVisitor {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_type_assertion(&mut self, expr: *mut AstExprTypeAssertion) -> bool {
    let expr_ref = unsafe { &*expr };
    let ast_resolved_types = unsafe { &*self.ast_resolved_types };

    if let Some(annot) = ast_resolved_types.find(&(expr_ref.annotation as *const _)) {
      self.apply_expected_type(*annot, expr_ref.expr as *const _);
    }

    true
  }
}
