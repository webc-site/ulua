use ulua_ast::records::{ast_node::AstNode, ast_visitor::AstVisitor};

#[derive(Debug, Clone, Default)]
pub struct CountAstNodes {
  pub count: u32,
}

impl AstVisitor for CountAstNodes {
  fn visit_node(&mut self, _node: &mut AstNode) -> bool {
    self.visit()
  }
}
