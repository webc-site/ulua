use core::ffi::c_void;

use crate::{
  records::{ast_stat_for::AstStatFor, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_stat_visit, ast_type_visit},
};

impl AstVisitable for AstStatFor {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_for(self as *const Self as *mut c_void) {
      unsafe {
        if !(*self.var).annotation.is_null() {
          ast_type_visit((*self.var).annotation, visitor);
        }

        ast_expr_visit(self.from, visitor);
        ast_expr_visit(self.to, visitor);

        if !self.step.is_null() {
          ast_expr_visit(self.step, visitor);
        }

        ast_stat_visit(self.body as *mut _, visitor);
      }
    }
  }
}

pub fn ast_stat_for_visit(this: &AstStatFor, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
