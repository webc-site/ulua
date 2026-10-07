use crate::records::ast_expr::AstExpr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprConstantBool {
  pub base: AstExpr,
  pub value: bool,
}
