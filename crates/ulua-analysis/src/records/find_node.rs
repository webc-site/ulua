use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_stat_function::AstStatFunction, ast_visitor::AstVisitor, position::Position,
  },
  visit,
};
#[derive(Debug, Clone)]
pub struct FindNode {
  pub(crate) pos: Position,
  pub(crate) document_end: Position,
  pub(crate) best: *mut AstNode,
}

impl FindNode {
  pub fn new(pos: Position, document_end: Position) -> Self {
    Self {
      pos,
      document_end,
      best: null_mut(),
    }
  }
}

impl AstVisitor for FindNode {
  fn visit_node(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstNode;
    unsafe {
      if (*node).location.contains(self.pos) {
        self.best = node;
        return true;
      }

      if (*node).location.end == self.document_end && self.pos >= self.document_end {
        self.best = node;
        return true;
      }
    }
    false
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    self.visit_node(node);
    let node = node as *mut AstStatFunction;
    unsafe {
      let name_node = (*node).name as *mut AstNode;
      let func_node = (*node).func as *mut AstNode;
      if (*name_node).location.contains(self.pos) {
        visit::ast_expr_visit((*node).name, self);
      } else if (*func_node).location.contains(self.pos) {
        visit::ast_expr_visit((*node).func as *mut AstExpr, self);
      }
    }
    false
  }

  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_node(node);
    let block = node as *mut AstStatBlock;
    unsafe {
      for stat in (*block).body.iter() {
        let stat = *stat;
        let stat_node = stat as *mut AstNode;
        if (*stat_node).location.end < self.pos {
          continue;
        }
        if (*stat_node).location.begin > self.pos {
          break;
        }
        visit::ast_stat_visit(stat, self);
      }
    }
    false
  }
}
