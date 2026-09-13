use ulua_ast::records::ast_expr::AstExpr;

use crate::records::find_expr_or_local::FindExprOrLocal;

impl FindExprOrLocal {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr(&mut self, expr: *mut AstExpr) -> bool {
    let location = unsafe { (*expr).base.location };
    if self.is_closer_match(location) {
      self.result.set_expr(expr);
      return true;
    }
    false
  }
}
