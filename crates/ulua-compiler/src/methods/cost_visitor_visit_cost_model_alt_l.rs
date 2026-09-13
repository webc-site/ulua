use core::ffi::c_void;

use ulua_ast::{records::ast_stat_block::AstStatBlock, visit::ast_stat_visit};

use crate::{functions::always_terminates::always_terminates, records::cost_visitor::CostVisitor};

pub fn visit_ast_stat_block(this: &mut CostVisitor, node: *mut c_void) -> bool {
  unsafe {
    if node.is_null() {
      return false;
    }

    let block = &*(node as *const AstStatBlock);

    for &stat in block.body.iter() {
      ast_stat_visit(stat, this);

      // C++ stops modelling a block once a statement unconditionally terminates
      // (return/break/continue, or an if whose branches all terminate); the rest
      // is dead. The placeholder always-false here over-counted post-return code.
      if always_terminates(&*this.constants, stat) {
        break;
      }
    }
  }

  false
}

impl CostVisitor {
  pub fn visit_ast_stat_block(&mut self, node: *mut c_void) -> bool {
    visit_ast_stat_block(self, node)
  }
}
