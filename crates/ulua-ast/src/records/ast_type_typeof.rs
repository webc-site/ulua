use crate::records::{ast_expr::AstExpr, ast_type::AstType, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeTypeof {
  pub base: AstType,
  /// cpp `AstExpr* expr`（`Ast.h:1295`）：唯一构造点 `Parser.cpp:3379` 直接接线
  /// `parseExpr()` 结果，`typeof(expr)` 文法必带表达式，恒非空。
  pub expr: Node<AstExpr>,
}
