use crate::records::{
  ast_expr_binary::AstExprBinaryOp,
  ast_expr_unary::{
    AstExprUnaryOp,
    AstExprUnaryOp::{Len, Minus, Not},
  },
};

/// 操作符符号（编译期字面量）：打印热路径零分配出口。
pub fn to_str(op: AstExprUnaryOp) -> &'static str {
  match op {
    Minus => "-",
    Not => "not",
    Len => "#",
  }
}

/// 操作符符号（编译期字面量）：打印热路径零分配出口。
pub fn to_str_binary(op: AstExprBinaryOp) -> &'static str {
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
