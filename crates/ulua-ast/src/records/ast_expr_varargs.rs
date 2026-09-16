use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprVarargs {
  pub base: AstExpr,
}

impl AstNodeClass for AstExprVarargs {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprVarargs");
}
