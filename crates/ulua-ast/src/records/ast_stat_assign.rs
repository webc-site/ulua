#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatAssign {
  pub base: AstStat,
  pub vars: AstArray<*mut AstExpr>,
  pub values: AstArray<*mut AstExpr>,
}

impl AstNodeClass for AstStatAssign {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatAssign");
}
use crate::{
  records::{ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};
