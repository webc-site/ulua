extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;

pub fn op_to_meta_table_entry(op: AstExprBinaryOp) -> String {
  match op {
    AstExprBinaryOp::CompareNe | AstExprBinaryOp::CompareEq => String::from("__eq"),
    AstExprBinaryOp::CompareLt | AstExprBinaryOp::CompareGe => String::from("__lt"),
    AstExprBinaryOp::CompareLe | AstExprBinaryOp::CompareGt => String::from("__le"),
    AstExprBinaryOp::Add => String::from("__add"),
    AstExprBinaryOp::Sub => String::from("__sub"),
    AstExprBinaryOp::Mul => String::from("__mul"),
    AstExprBinaryOp::Div => String::from("__div"),
    AstExprBinaryOp::FloorDiv => String::from("__idiv"),
    AstExprBinaryOp::Mod => String::from("__mod"),
    AstExprBinaryOp::Pow => String::from("__pow"),
    AstExprBinaryOp::Concat => String::from("__concat"),
    _ => String::new(),
  }
}
