use core::ffi::c_void;

use crate::{
  records::{ast_expr_binary::AstExprBinary, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprBinary {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_binary(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.left, visitor);
        ast_expr_visit(self.right, visitor);
      }
    }
  }
}

pub fn ast_expr_binary_visit(this: &AstExprBinary, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
