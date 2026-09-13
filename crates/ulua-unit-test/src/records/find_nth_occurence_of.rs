use core::ffi::c_void;

use ulua_ast::records::{ast_node::AstNode, ast_visitor::AstVisitor};

use crate::records::nth::Nth;
#[derive(Debug, Clone)]
pub struct FindNthOccurenceOf {
  pub requested_nth: Nth,
  pub current_occurrence: i32,
  pub the_node: *mut AstNode,
}

impl AstVisitor for FindNthOccurenceOf {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    self.check_it(node as *mut AstNode)
  }

  fn visit_type(&mut self, node: *mut c_void) -> bool {
    self.check_it(node as *mut AstNode)
  }

  fn visit_type_pack(&mut self, node: *mut c_void) -> bool {
    self.check_it(node as *mut AstNode)
  }
}

impl FindNthOccurenceOf {
  pub(crate) fn check_it(&mut self, n: *mut AstNode) -> bool {
    let node = unsafe { &*n };
    if node.class_index == self.requested_nth.class_index {
      self.current_occurrence += 1;
      if self.current_occurrence == self.requested_nth.nth {
        self.the_node = n;
        return false;
      }
    }
    true
  }
}
