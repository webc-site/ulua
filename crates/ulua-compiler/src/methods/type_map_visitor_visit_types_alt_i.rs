use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_index_expr::AstExprIndexExpr},
  visit,
};

use crate::records::type_map_visitor::TypeMapVisitor;

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_index_expr(&mut self, node: *mut AstExprIndexExpr) -> bool {
    unsafe {
      let expr = (*node).expr;
      let index = (*node).index;

      visit::ast_expr_visit(expr, self);
      visit::ast_expr_visit(index, self);

      if !self.try_get_table_indexer(expr).is_null() {
        let indexer = self.try_get_table_indexer(expr);
        let result_type = (*indexer).result_type;
        self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, result_type);
      }
    }

    false
  }
}
