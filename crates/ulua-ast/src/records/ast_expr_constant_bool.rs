use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprConstantBool {
  pub base: AstExpr,
  pub value: bool,
}

impl AstNodeClass for AstExprConstantBool {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprConstantBool");
}
