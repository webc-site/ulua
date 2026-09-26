use crate::records::{ast_expr::AstExpr, ast_type::AstType, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstExprTypeAssertion {
  pub base: AstExpr,
  /// cpp `AstExpr* expr`（Ast.h:657）：构造必传（Ast.h:653），Ast.cpp:508 visit 端无
  /// 守卫解引用，parse_simple_expr 子表达式恒非空 → 恒非空。
  pub expr: Node<AstExpr>,
  /// cpp `AstType* annotation`（Ast.h:658）：构造端 Parser.cpp:3911 解引用
  /// `annotation->location` 且 Ast.cpp:509 visit 无守卫，parse_type 恒回非空节点
  /// （错误分支亦产 AstTypeError）→ 恒非空。
  pub annotation: Node<AstType>,
}
