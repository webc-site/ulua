use core::ffi::c_void;

use ulua_ast::records::ast_stat_return::AstStatReturn;

use crate::records::visitor::Visitor;
impl Visitor {
  pub fn visit_ast_stat_return(&mut self, node: *mut c_void) -> bool {
    let node = node as *mut AstStatReturn;

    unsafe {
      if self.result.is_null() && !(*node).list.is_empty() {
        self.result = node;
      }
    }

    false
  }
}
