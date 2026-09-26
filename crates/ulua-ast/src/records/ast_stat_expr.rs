use crate::records::{ast_expr::AstExpr, ast_stat::AstStat, node_handle::Node};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatExpr {
  pub base: AstStat,
  /// cpp `AstExpr* expr`（Ast.h:871）：ctor 必传（Ast.h:867），visit 端无守卫
  /// 下钻（Ast.cpp:756），parser 端 `parse_primary_expr` 恒返回 arena 非空节点
  /// → 恒非空 Node。
  pub expr: Node<AstExpr>,
}
