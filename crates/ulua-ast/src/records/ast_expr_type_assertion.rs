use crate::{
  records::{ast_expr::AstExpr, ast_type::AstType},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprTypeAssertion {
  pub base: AstExpr,
  pub expr: *mut AstExpr,
  pub annotation: *mut AstType,
}

impl AstNodeClass for AstExprTypeAssertion {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprTypeAssertion");
}
