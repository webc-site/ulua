use crate::records::ast_expr::AstExpr;

#[repr(C)]
#[derive(Debug)]
pub struct AstExprVarargs {
  pub base: AstExpr,
}
