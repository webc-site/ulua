use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprIfElse {
  pub base: AstExpr,
  pub condition: *mut AstExpr,
  pub has_then: bool,
  pub true_expr: *mut AstExpr,
  pub has_else: bool,
  pub false_expr: *mut AstExpr,
}

impl AstNodeClass for AstExprIfElse {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprIfElse");
}
