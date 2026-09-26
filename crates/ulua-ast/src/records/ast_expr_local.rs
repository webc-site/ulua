use crate::records::{ast_expr::AstExpr, ast_local::AstLocal, node_handle::Node};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprLocal {
  pub base: AstExpr,
  /// cpp `AstLocal* local`（Ast.h:424）：构造端 Parser.cpp:3734 由
  /// `if (value && *value)` 守卫后才 alloc（且解引用 `local->functionDepth`），
  /// 未命中即产 AstExprGlobal → 恒非空。
  pub local: Node<AstLocal>,
  pub upvalue: bool,
}
