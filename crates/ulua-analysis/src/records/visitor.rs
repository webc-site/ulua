use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::records::{ast_stat_return::AstStatReturn, ast_visitor::AstVisitor};
#[derive(Debug, Clone)]
pub struct Visitor {
  pub(crate) result: *mut AstStatReturn,
}

impl Visitor {
  pub fn new() -> Self {
    Self { result: null_mut() }
  }
}

impl Default for Visitor {
  fn default() -> Self {
    Self::new()
  }
}

impl AstVisitor for Visitor {
  fn visit_expr(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatReturn;
    unsafe {
      if self.result.is_null() && !(*node).list.is_empty() {
        self.result = node;
      }
    }
    false
  }

  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
