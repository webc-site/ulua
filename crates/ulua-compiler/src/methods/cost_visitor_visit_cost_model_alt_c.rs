use core::ffi::c_void;

use ulua_ast::records::ast_stat_for_in::AstStatForIn;

use crate::records::{cost::Cost, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn visit_ast_stat_for_in(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatForIn);

      for expr_ptr in node.values.as_slice() {
        let cost = self.model(*expr_ptr);
        // C++ `result += model(...)` — saturating add (Cost::operator+=).
        self.result.add_assign(&cost);
      }

      self.loop_item(
        node.body,
        Cost {
          model: 1,
          constant: 0,
        },
        3,
      );
    }

    false
  }
}
