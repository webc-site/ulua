use crate::records::{ast_expr::AstExpr, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprGroup {
  pub base: AstExpr,
  /// cpp `AstExpr* expr`（Ast.h:316）：构造端 parse_prefix_expr 传入 parse_expr 的
  /// arena 分配结果，visit 端无守卫解引用（Ast.cpp `expr->visit(visitor)`），恒非空。
  pub expr: Node<AstExpr>,
}
