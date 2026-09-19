use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::lint_comparison_precedence::LintComparisonPrecedence;
impl LintComparisonPrecedence {
  pub fn is_not(&self, node: *mut AstExpr) -> bool {
    let expr = unsafe { ast_node_as::<AstExprUnary>(node as *mut AstNode) };
    !expr.is_null() && unsafe { (*expr).op == AstExprUnaryOp::Not }
  }
}
