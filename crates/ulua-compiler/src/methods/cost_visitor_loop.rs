use core::ffi::c_void;

use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::records::{cost::Cost, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn loop_item(&mut self, body: *mut AstStatBlock, iter_cost: Cost, factor: i32) {
    let before = self.result;

    self.result = Cost::default();

    if !body.is_null() {
      self.visit_ast_stat_block(body as *mut c_void);
    }

    self.result = before.operator_add(&self.result.operator_add(&iter_cost).operator_mul(factor));
  }
}

pub fn cost_visitor_loop(
  this: &mut CostVisitor,
  body: *mut AstStatBlock,
  iter_cost: Cost,
  factor: i32,
) {
  this.loop_item(body, iter_cost, factor);
}
