#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatAssign {
  pub base: AstStat,
  pub vars: AstArray<*mut AstExpr>,
  pub values: AstArray<*mut AstExpr>,
}

use crate::records::{ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat};
