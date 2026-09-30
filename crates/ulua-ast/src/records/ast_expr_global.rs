use crate::records::{ast_expr::AstExpr, ast_name::AstName};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprGlobal {
  pub base: AstExpr,
  pub name: AstName,
}
