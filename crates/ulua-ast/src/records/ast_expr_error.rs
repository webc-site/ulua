use crate::records::{ast_array::AstArray, ast_expr::AstExpr};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprError {
  pub base: AstExpr,
  pub expressions: AstArray<*mut AstExpr>,
  pub message_index: u32,
}
