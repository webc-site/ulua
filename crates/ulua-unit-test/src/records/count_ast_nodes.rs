use core::ffi::c_void;

use ulua_ast::records::{ast_node::AstNode, ast_visitor::AstVisitor};
#[derive(Debug, Clone, Default)]
pub struct CountAstNodes {
  pub count: u32,
}

impl AstVisitor for CountAstNodes {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    Self::visit(self, node as *mut AstNode)
  }
}
