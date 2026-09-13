use crate::{
  records::{ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatError {
  pub base: AstStat,
  pub expressions: AstArray<*mut AstExpr>,
  pub statements: AstArray<*mut AstStat>,
  pub message_index: u32,
}

impl AstNodeClass for AstStatError {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatError");
}
