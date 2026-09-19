use crate::records::ast_expr_unary::{
  AstExprUnaryOp,
  AstExprUnaryOp::{Len, Minus, Not},
};

/// 操作符符号（编译期字面量）：打印热路径零分配出口。
pub fn to_str(op: AstExprUnaryOp) -> &'static str {
  match op {
    Minus => "-",
    Not => "not",
    Len => "#",
  }
}
