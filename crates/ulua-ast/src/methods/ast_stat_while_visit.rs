use core::ffi::c_void;

use crate::{
  records::{ast_stat_while::AstStatWhile, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_stat_visit},
};

impl AstVisitable for AstStatWhile {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_while(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.condition, visitor);
        ast_stat_visit(self.body as *mut _, visitor);
      }
    }
  }
}

pub fn ast_stat_while_visit(this: &AstStatWhile, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
