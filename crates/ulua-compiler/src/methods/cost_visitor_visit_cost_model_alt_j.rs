use core::ffi::c_void;

use crate::records::{cost::Cost, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn visit_ast_stat_break(&mut self, _node: *mut c_void) -> bool {
    self.result.add_assign(&Cost::new(1, 0));
    false
  }
}
