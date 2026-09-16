use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{functions::similar::similar, records::lint_table_operations::LintTableOperations};
impl LintTableOperations {
  /// # Safety
  /// 调用方须保证 `expr、`table` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn is_length(&mut self, expr: *mut AstExpr, table: *mut AstExpr) -> bool {
    let n = unsafe { ast_node_as::<AstExprUnary>(expr as *mut AstNode) };
    if n.is_null() {
      return false;
    }
    let n_ref = unsafe { &*n };
    n_ref.op == AstExprUnaryOp::Len && unsafe { similar(n_ref.expr, table) }
  }
}
