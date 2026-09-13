use core::ffi::c_void;

use crate::records::visitor::Visitor;

impl Visitor {
  pub fn visit_ast_expr(&mut self, _node: *mut c_void) -> bool {
    let _ = _node;
    false
  }
}
