use core::ffi::c_void;

use crate::{
  records::{ast_expr_if_else::AstExprIfElse, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprIfElse {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_if_else(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.condition, visitor);
        ast_expr_visit(self.true_expr, visitor);
        ast_expr_visit(self.false_expr, visitor);
      }
    }
  }
}

pub fn ast_expr_if_else_visit(this: &AstExprIfElse, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
