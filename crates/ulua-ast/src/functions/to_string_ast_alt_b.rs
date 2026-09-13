use alloc::string::String;

use crate::records::ast_expr_binary::AstExprBinaryOp;

pub fn to_string(op: AstExprBinaryOp) -> String {
  match op {
    AstExprBinaryOp::Add => String::from("+"),
    AstExprBinaryOp::Sub => String::from("-"),
    AstExprBinaryOp::Mul => String::from("*"),
    AstExprBinaryOp::Div => String::from("/"),
    AstExprBinaryOp::FloorDiv => String::from("//"),
    AstExprBinaryOp::Mod => String::from("%"),
    AstExprBinaryOp::Pow => String::from("^"),
    AstExprBinaryOp::Concat => String::from(".."),
    AstExprBinaryOp::CompareNe => String::from("~="),
    AstExprBinaryOp::CompareEq => String::from("=="),
    AstExprBinaryOp::CompareLt => String::from("<"),
    AstExprBinaryOp::CompareLe => String::from("<="),
    AstExprBinaryOp::CompareGt => String::from(">"),
    AstExprBinaryOp::CompareGe => String::from(">="),
    AstExprBinaryOp::And => String::from("and"),
    AstExprBinaryOp::Or => String::from("or"),
    _ => {
      ulua_common::LUAU_ASSERT!(false);
      String::new()
    }
  }
}

// Pinned overload name advertised by the dependency cards.
pub use to_string as to_string_ast_expr_binary_op;
