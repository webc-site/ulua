use core::ffi::c_void;

use crate::{
  records::{ast_stat_continue::AstStatContinue, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstStatContinue {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_stat_continue(self as *const Self as *mut c_void);
  }
}

pub fn ast_stat_continue_visit(this: &AstStatContinue, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
