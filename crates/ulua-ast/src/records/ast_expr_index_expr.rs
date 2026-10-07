use crate::records::{ast_expr::AstExpr, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprIndexExpr {
  pub base: AstExpr,
  /// cpp `AstExpr* expr/index`（Ast.h:508-509）：构造必传（Ast.h:502），Ast.cpp visit
  /// 端两端均无守卫解引用，parser 基表达式与索引表达式恒非空 → 恒非空。
  pub expr: Node<AstExpr>,
  pub index: Node<AstExpr>,
}
