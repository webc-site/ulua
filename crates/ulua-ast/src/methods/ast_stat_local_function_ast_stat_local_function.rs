use crate::records::{
  ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_stat::AstStat,
  ast_stat_local_function::AstStatLocalFunction, location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstStatLocalFunction,
  AstStat,
  location: Location,
  name: Node<AstLocal>,
  func: Node<AstExprFunction>,
  is_const: bool,
);
