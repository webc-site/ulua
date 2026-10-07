use crate::records::{ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatReturn {
  pub base: AstStat,
  pub list: AstArray<*mut AstExpr>,
}
