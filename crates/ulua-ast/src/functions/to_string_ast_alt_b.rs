use crate::records::ast_expr_binary::AstExprBinaryOp;

/// 操作符符号（编译期字面量）：打印热路径零分配出口。
pub fn to_str(op: AstExprBinaryOp) -> &'static str {
  match op {
    AstExprBinaryOp::Add => "+",
    AstExprBinaryOp::Sub => "-",
    AstExprBinaryOp::Mul => "*",
    AstExprBinaryOp::Div => "/",
    AstExprBinaryOp::FloorDiv => "//",
    AstExprBinaryOp::Mod => "%",
    AstExprBinaryOp::Pow => "^",
    AstExprBinaryOp::Concat => "..",
    AstExprBinaryOp::CompareNe => "~=",
    AstExprBinaryOp::CompareEq => "==",
    AstExprBinaryOp::CompareLt => "<",
    AstExprBinaryOp::CompareLe => "<=",
    AstExprBinaryOp::CompareGt => ">",
    AstExprBinaryOp::CompareGe => ">=",
    AstExprBinaryOp::And => "and",
    AstExprBinaryOp::Or => "or",
    _ => {
      ulua_common::LUAU_ASSERT!(false);
      ""
    }
  }
}
