use ulua_ast::records::ast_node::AstNode;
use ulua_common::collections::HashSet;
#[derive(Debug, Clone)]
pub struct AstVisitorTracking {
  pub visited_nodes: Vec<*mut AstNode>,
  pub seen: HashSet<usize>,
}
