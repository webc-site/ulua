use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;

/// 比较运算符族（CompareNe/CompareEq/CompareLt/CompareLe/CompareGt/CompareGe）
/// 的单一判定点：条件跳转、二元分发与类型登记路径都按「是否比较运算」分流，
/// 各处原本重复列举六个变体，收口于此。
#[inline]
pub(crate) const fn is_compare_op(op: AstExprBinaryOp) -> bool {
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
