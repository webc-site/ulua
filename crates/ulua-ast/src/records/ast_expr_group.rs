use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprGroup {
  pub base: AstExpr,
  pub expr: *mut AstExpr,
}

impl AstNodeClass for AstExprGroup {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprGroup");
}
