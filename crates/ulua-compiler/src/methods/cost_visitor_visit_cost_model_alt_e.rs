use core::ffi::c_void;

use ulua_ast::records::ast_stat_repeat::AstStatRepeat;

use crate::records::cost_visitor::CostVisitor;

impl CostVisitor {
  pub fn visit_ast_stat_repeat(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatRepeat);

      let condition = self.model(node.condition);

      // C++ `loop(node->body, condition)` uses the default factor of 3, not 1.
      self.loop_item(node.body, condition, 3);
    }

    false
  }
}
