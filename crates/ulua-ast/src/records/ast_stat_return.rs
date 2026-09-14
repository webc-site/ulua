use crate::{
  records::{ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatReturn {
  pub base: AstStat,
  pub list: AstArray<*mut AstExpr>,
}

impl AstNodeClass for AstStatReturn {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatReturn");
}
