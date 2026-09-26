use ulua_ast::records::ast_expr_binary::AstExprBinaryOp;

pub fn op_to_meta_table_entry(op: AstExprBinaryOp) -> &'static str {
  match op {
    AstExprBinaryOp::CompareNe | AstExprBinaryOp::CompareEq => "__eq",
    AstExprBinaryOp::CompareLt | AstExprBinaryOp::CompareGe => "__lt",
    AstExprBinaryOp::CompareLe | AstExprBinaryOp::CompareGt => "__le",
    AstExprBinaryOp::Add => "__add",
    AstExprBinaryOp::Sub => "__sub",
    AstExprBinaryOp::Mul => "__mul",
    AstExprBinaryOp::Div => "__div",
    AstExprBinaryOp::FloorDiv => "__idiv",
    AstExprBinaryOp::Mod => "__mod",
    AstExprBinaryOp::Pow => "__pow",
    AstExprBinaryOp::Concat => "__concat",
    _ => "",
  }
}
