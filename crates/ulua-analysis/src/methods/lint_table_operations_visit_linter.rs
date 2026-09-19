//! C++ `LintTableOperations::visit(AstExprUnary*)` (`Analysis/src/Linter.cpp:2610`).

use ulua_ast::records::{
  ast_expr::AstExpr,
  ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
};

use crate::records::lint_table_operations::LintTableOperations;

impl LintTableOperations {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
    unsafe {
      if (*node).op == AstExprUnaryOp::Len {
        self.check_indexer(&*(node as *mut AstExpr), &*(*node).expr, "#");
      }
    }

    true
  }
}
