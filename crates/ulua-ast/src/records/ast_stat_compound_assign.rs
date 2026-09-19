use crate::{
  records::{ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_stat::AstStat},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatCompoundAssign {
  pub base: AstStat,
  pub op: AstExprBinaryOp,
  pub var: *mut AstExpr,
  pub value: *mut AstExpr,
}

impl AstNodeClass for AstStatCompoundAssign {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatCompoundAssign");
}
