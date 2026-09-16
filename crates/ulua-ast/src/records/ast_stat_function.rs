use crate::{
  records::{ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatFunction {
  pub base: AstStat,
  pub name: *mut AstExpr,
  pub func: *mut AstExprFunction,
}

impl AstNodeClass for AstStatFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatFunction");
}
