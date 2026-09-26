use crate::records::{
  ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_while::AstStatWhile,
  location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstStatWhile,
  AstStat,
  location: Location,
  condition: Node<AstExpr>,
  body: Node<AstStatBlock>,
  has_do: bool,
  do_location: Location,
);
