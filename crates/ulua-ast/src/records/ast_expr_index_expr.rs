use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprIndexExpr {
  pub base: AstExpr,
  pub expr: *mut AstExpr,
  pub index: *mut AstExpr,
}

impl AstNodeClass for AstExprIndexExpr {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprIndexExpr");
}
