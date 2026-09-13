use ulua_ast::records::ast_node::AstNode;

use crate::records::count_ast_nodes::CountAstNodes;

impl CountAstNodes {
  pub fn visit(&mut self, _node: *mut AstNode) -> bool {
    self.count += 1;
    true
  }
}
