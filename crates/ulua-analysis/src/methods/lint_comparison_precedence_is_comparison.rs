use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;

use crate::records::lint_comparison_precedence::LintComparisonPrecedence;

impl LintComparisonPrecedence {
  pub fn is_comparison(&self, op: AstExprBinaryOp) -> bool {
    matches!(
      op,
      AstExprBinaryOp::CompareNe
        | AstExprBinaryOp::CompareEq
        | AstExprBinaryOp::CompareLt
        | AstExprBinaryOp::CompareLe
        | AstExprBinaryOp::CompareGt
        | AstExprBinaryOp::CompareGe
    )
  }
}
