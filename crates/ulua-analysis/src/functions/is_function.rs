use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction,
  },
  rtti::ast_node_is,
};
pub fn is_function(stat: &AstStat) -> bool {
  let node = stat as *const AstStat as *mut AstNode;
  ast_node_is::<AstStatFunction>(node) || ast_node_is::<AstStatLocalFunction>(node)
}
