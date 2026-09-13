use crate::{
  records::{ast_array::AstArray, ast_expr::AstExpr},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprError {
  pub base: AstExpr,
  pub expressions: AstArray<*mut AstExpr>,
  pub message_index: u32,
}

impl AstNodeClass for AstExprError {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprError");
}
