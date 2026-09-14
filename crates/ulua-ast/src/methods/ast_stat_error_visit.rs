use core::ffi::c_void;

use crate::{
  records::{ast_stat_error::AstStatError, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_stat_visit},
};

impl AstVisitable for AstStatError {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_stat_error(self as *const Self as *mut c_void) {
      for &expression in self.expressions.iter() {
        unsafe {
          ast_expr_visit(expression, visitor);
        }
      }

      for &statement in self.statements.iter() {
        unsafe {
          ast_stat_visit(statement, visitor);
        }
      }
    }
  }
}

pub fn ast_stat_error_visit<V: AstVisitor + ?Sized>(this: &AstStatError, visitor: &mut V) {
  this.visit(visitor);
}
