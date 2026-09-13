use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprConstantNil {
  pub base: AstExpr,
}

impl AstNodeClass for AstExprConstantNil {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprConstantNil");
}
