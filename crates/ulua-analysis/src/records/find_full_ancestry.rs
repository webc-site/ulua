use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat_function::AstStatFunction,
    ast_visitor::AstVisitor, position::Position,
  },
  visit::ast_expr_visit,
};
#[derive(Debug, Clone)]
pub struct FindFullAncestry {
  pub(crate) nodes: Vec<*mut AstNode>,
  pub(crate) pos: Position,
  pub(crate) document_end: Position,
  pub(crate) include_types: bool,
}

impl AstVisitor for FindFullAncestry {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstNode;
    let node_ref = unsafe { &*node };

    if node_ref.location.contains(self.pos) {
      self.nodes.push(node);
      return true;
    }

    if node_ref.location.end == self.document_end && self.pos >= self.document_end {
      self.nodes.push(node);
      return true;
    }

    false
  }

  fn visit_type(&mut self, node: *mut c_void) -> bool {
    if self.include_types {
      self.visit_node(node)
    } else {
      false
    }
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatFunction;
    let node_ref = unsafe { &*node };

    self.visit_node(node as *mut AstNode as *mut c_void);

    if unsafe { (*node_ref.name).base.location.contains(self.pos) } {
      unsafe {
        ast_expr_visit(node_ref.name, self);
      }
    } else if unsafe { (*node_ref.func).base.base.location.contains(self.pos) } {
      unsafe {
        ast_expr_visit(node_ref.func as *mut AstExpr, self);
      }
    }

    false
  }
}
