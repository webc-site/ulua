use crate::records::{ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatError {
  pub base: AstStat,
  pub expressions: AstArray<*mut AstExpr>,
  pub statements: AstArray<*mut AstStat>,
  pub message_index: u32,
}
