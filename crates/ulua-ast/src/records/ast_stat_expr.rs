use crate::{
  records::{ast_expr::AstExpr, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatExpr {
  pub base: AstStat,
  pub expr: *mut AstExpr,
}

impl AstNodeClass for AstStatExpr {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatExpr");
}
