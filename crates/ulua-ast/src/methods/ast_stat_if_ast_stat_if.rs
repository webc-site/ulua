use crate::records::{
  ast_expr::AstExpr,
  ast_stat::AstStat,
  ast_stat_block::AstStatBlock,
  ast_stat_if::AstStatIf,
  location::Location,
  node_handle::{Node, OptNode},
};

impl_ast_node_new!(
  AstStatIf,
  AstStat,
  location: Location,
  condition: Node<AstExpr>,
  thenbody: Node<AstStatBlock>,
  elsebody: OptNode<AstStat>,
  then_location: Option<Location>,
  else_location: Option<Location>,
);
