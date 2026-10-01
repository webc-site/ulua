use crate::records::{
  ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_stat::AstStat, node_handle::Node,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatCompoundAssign {
  pub base: AstStat,
  pub op: AstExprBinaryOp,
  /// cpp `AstExpr* var`（Ast.h:980）：ctor 必传（Ast.h:975/Ast.cpp:887），visit 端
  /// 无守卫下钻（Ast.cpp:896），parser 端左值由 parse_primary_expr 恒非空产出
  /// → 恒非空 Node。
  pub var: Node<AstExpr>,
  /// cpp `AstExpr* value`（Ast.h:981），非空口径同 [`Self::var`]（Ast.cpp:888/897）。
  pub value: Node<AstExpr>,
}
