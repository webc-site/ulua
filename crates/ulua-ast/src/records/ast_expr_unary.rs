use crate::records::{ast_expr::AstExpr, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprUnary {
  pub base: AstExpr,
  pub op: AstExprUnaryOp,
  /// cpp `AstExpr* expr`（Ast.h:605）：构造必传（Ast.h:601），visit 端无守卫解引用
  /// （Ast.cpp `expr->visit(visitor)`），parser 端 parse_expr 恒非空 → 恒非空。
  pub expr: Node<AstExpr>,
}

#[repr(u32)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr, strum::IntoStaticStr, strum::Display,
)]
pub enum AstExprUnaryOp {
  Not,
  Minus,
  Len,
}
