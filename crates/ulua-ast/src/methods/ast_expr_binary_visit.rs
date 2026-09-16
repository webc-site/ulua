use core::ffi::c_void;

use crate::{
  records::{ast_expr_binary::AstExprBinary, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprBinary {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_expr_binary(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.left, visitor);
        ast_expr_visit(self.right, visitor);
      }
    }
  }
}

pub fn ast_expr_binary_visit<V: AstVisitor + ?Sized>(this: &AstExprBinary, visitor: &mut V) {
  this.visit(visitor);
}
