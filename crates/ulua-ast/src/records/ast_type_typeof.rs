use crate::records::{ast_expr::AstExpr, ast_type::AstType};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeTypeof {
  pub base: AstType,
  pub expr: *mut AstExpr,
}
