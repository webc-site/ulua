use crate::records::{
  ast_expr::AstExpr, ast_name::AstName, location::Location, node_handle::Node, position::Position,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprIndexName {
  pub base: AstExpr,
  /// cpp `AstExpr* expr`（Ast.h:492）：构造必传（Ast.h:483），Ast.cpp visit 端无守卫
  /// 解引用，parser 基表达式恒非空 → 恒非空。
  pub expr: Node<AstExpr>,
  pub index: AstName,
  pub index_location: Location,
  pub op_position: Position,
  pub op: u8,
}
