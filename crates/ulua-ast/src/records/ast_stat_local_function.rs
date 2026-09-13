#[repr(C)]
#[derive(Debug)]
pub struct AstStatLocalFunction {
  pub base: AstStat,
  pub name: *mut AstLocal,
  pub func: *mut AstExprFunction,
  pub is_const: bool,
}

impl AstNodeClass for AstStatLocalFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatLocalFunction");
}
use crate::{
  records::{ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};
