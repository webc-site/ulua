use alloc::string::String;

use crate::records::ast_expr_unary::{
  AstExprUnaryOp,
  AstExprUnaryOp::{Len, Minus, Not},
};

pub fn to_string(op: AstExprUnaryOp) -> String {
  match op {
    Minus => String::from("-"),
    Not => String::from("not"),
    Len => String::from("#"),
  }
}
