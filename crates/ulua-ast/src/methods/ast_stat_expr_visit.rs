use core::ffi::c_void;

use crate::{
  records::{ast_stat_expr::AstStatExpr, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstStatExpr {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_stat_expr(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
    }
  }
}

pub fn ast_stat_expr_visit<V: AstVisitor + ?Sized>(this: &AstStatExpr, visitor: &mut V) {
  this.visit(visitor);
}
