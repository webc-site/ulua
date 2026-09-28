use core::ptr::from_mut;

use ulua_ast::records::{
  ast_node::AstNode, ast_type::AstType, ast_type_pack::AstTypePack, ast_visitor::AstVisitor,
};

use crate::records::nth::Nth;
#[derive(Debug, Clone)]
pub struct FindNthOccurenceOf {
  pub requested_nth: Nth,
  pub current_occurrence: i32,
  pub the_node: *mut AstNode,
}

impl AstVisitor for FindNthOccurenceOf {
  fn visit_node(&mut self, node: &mut AstNode) -> bool {
    self.check_it(node)
  }

  fn visit_type(&mut self, node: &mut AstType) -> bool {
    self.check_it(&mut node.base)
  }

  fn visit_type_pack(&mut self, node: &mut AstTypePack) -> bool {
    self.check_it(&mut node.base)
  }
}

impl FindNthOccurenceOf {
  pub(crate) fn check_it(&mut self, node: &mut AstNode) -> bool {
    if node.class_index == self.requested_nth.class_index {
      self.current_occurrence += 1;
      if self.current_occurrence == self.requested_nth.nth {
        self.the_node = from_mut(node);
        return false;
      }
    }
    true
  }
}
