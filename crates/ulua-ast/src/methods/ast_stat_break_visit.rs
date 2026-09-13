use core::ffi::c_void;

use crate::{
  records::{ast_stat_break::AstStatBreak, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstStatBreak {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_stat_break(self as *const Self as *mut c_void);
  }
}

pub fn ast_stat_break_visit(this: &AstStatBreak, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
