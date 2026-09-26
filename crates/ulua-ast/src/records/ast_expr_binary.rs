use crate::records::{ast_expr::AstExpr, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprBinary {
  pub base: AstExpr,
  pub op: AstExprBinaryOp,
  /// cpp `AstExpr* left/right`（Ast.h:643-644）：构造必传（Ast.h:639），Ast.cpp visit
  /// 端两端均无守卫解引用，parse_expr 操作数恒非空 → 恒非空。
  pub left: Node<AstExpr>,
  pub right: Node<AstExpr>,
}

#[repr(u32)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr, strum::IntoStaticStr, strum::Display,
)]
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
