use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;

pub fn is_comparison_op(op: AstExprBinaryOp) -> bool {
  matches!(
    op,
    AstExprBinaryOp::CompareNe
      | AstExprBinaryOp::CompareEq
      | AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::CompareGt
      | AstExprBinaryOp::CompareLe
      | AstExprBinaryOp::CompareLt
  )
}
