use core::ffi::c_void;

use crate::{
  records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_stat_visit},
};

impl AstVisitable for AstStatBlock {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_block(self as *const Self as *mut c_void) {
      for stat_ptr in self.body.iter() {
        unsafe {
          ast_stat_visit(*stat_ptr, visitor);
        }
      }
    }
  }
}

pub fn ast_stat_block_visit(this: &AstStatBlock, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
