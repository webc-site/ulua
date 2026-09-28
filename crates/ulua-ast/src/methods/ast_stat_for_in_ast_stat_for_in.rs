use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat,
  ast_stat_block::AstStatBlock, ast_stat_for_in::AstStatForIn, location::Location,
  node_handle::Node,
};

impl_ast_node_new!(
  AstStatForIn,
  AstStat,
  location: Location,
  vars: AstArray<*mut AstLocal>,
  values: AstArray<*mut AstExpr>,
  body: Node<AstStatBlock>,
  has_in: bool,
  in_location: Location,
  has_do: bool,
  do_location: Location,
);
