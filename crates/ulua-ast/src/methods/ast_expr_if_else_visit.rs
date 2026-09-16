use core::ffi::c_void;

use crate::{
  records::{ast_expr_if_else::AstExprIfElse, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprIfElse {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_expr_if_else(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.condition, visitor);
        ast_expr_visit(self.true_expr, visitor);
        ast_expr_visit(self.false_expr, visitor);
      }
    }
  }
}

pub fn ast_expr_if_else_visit<V: AstVisitor + ?Sized>(this: &AstExprIfElse, visitor: &mut V) {
  this.visit(visitor);
}
