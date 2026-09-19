use core::ffi::c_void;

use crate::{
  records::{ast_expr_unary::AstExprUnary, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprUnary {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_expr_unary(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
    }
  }
}
