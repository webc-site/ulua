use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprUnary {
  pub base: AstExpr,
  pub op: AstExprUnaryOp,
  pub expr: *mut AstExpr,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstExprUnaryOp {
  Not,
  Minus,
  Len,
}

impl AstNodeClass for AstExprUnary {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprUnary");
}
