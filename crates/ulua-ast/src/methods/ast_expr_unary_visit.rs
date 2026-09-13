use core::ffi::c_void;

use crate::{
  records::{ast_expr_unary::AstExprUnary, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprUnary {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_unary(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
    }
  }
}

pub fn ast_expr_unary_visit(this: &AstExprUnary, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
