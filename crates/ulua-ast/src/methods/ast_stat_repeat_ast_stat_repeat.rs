use crate::records::{
  ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock,
  ast_stat_repeat::AstStatRepeat, location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstStatRepeat,
  AstStat,
  location: Location,
  condition: Node<AstExpr>,
  body: Node<AstStatBlock>,
  deprecated_has_until: bool,
);
