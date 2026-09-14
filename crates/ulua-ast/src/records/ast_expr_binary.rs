use crate::{
  records::ast_expr::AstExpr,
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprBinary {
  pub base: AstExpr,
  pub op: AstExprBinaryOp,
  pub left: *mut AstExpr,
  pub right: *mut AstExpr,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstExprBinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  FloorDiv,
  Mod,
  Pow,
  Concat,
  CompareNe,
  CompareEq,
  CompareLt,
  CompareLe,
  CompareGt,
  CompareGe,
  And,
  Or,
  OpCount,
}

impl AstNodeClass for AstExprBinary {
  const CLASS_INDEX: i32 = ast_rtti_index("AstExprBinary");
}
