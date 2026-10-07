use crate::records::{
  ast_expr::AstExpr,
  ast_local::AstLocal,
  ast_stat::AstStat,
  ast_stat_block::AstStatBlock,
  ast_stat_for::AstStatFor,
  location::Location,
  node_handle::{Node, OptNode},
};

impl_ast_node_new!(
  AstStatFor,
  AstStat,
  location: Location,
  var: Node<AstLocal>,
  from: Node<AstExpr>,
  to: Node<AstExpr>,
  step: OptNode<AstExpr>,
  body: Node<AstStatBlock>,
  has_do: bool,
  do_location: Location,
);
