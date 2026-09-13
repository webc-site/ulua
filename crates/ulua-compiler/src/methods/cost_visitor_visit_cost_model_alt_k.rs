use core::ffi::c_void;

use ulua_ast::records::ast_stat_continue::AstStatContinue;

use crate::records::{cost::Cost, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn visit_ast_stat_continue(&mut self, node: *mut c_void) -> bool {
    let _node = node as *mut AstStatContinue;
    self.result.add_assign(&Cost::new(1, 0));

    false
  }
}
