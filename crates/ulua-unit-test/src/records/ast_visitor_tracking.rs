use std::collections::HashSet;

use ulua_ast::records::ast_node::AstNode;
#[derive(Debug, Clone)]
pub struct AstVisitorTracking {
  pub visited_nodes: Vec<*mut AstNode>,
  pub seen: HashSet<usize>,
}
