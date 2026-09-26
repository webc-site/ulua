use crate::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_stat::AstStat,
  ast_stat_function::AstStatFunction, location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstStatFunction,
  AstStat,
  location: Location,
  name: Node<AstExpr>,
  func: Node<AstExprFunction>,
);
