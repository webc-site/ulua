use core::ffi::c_void;

use crate::{
  records::{ast_expr_group::AstExprGroup, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprGroup {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_expr_group(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
    }
  }
}

pub fn ast_expr_group_visit<V: AstVisitor + ?Sized>(this: &AstExprGroup, visitor: &mut V) {
  this.visit(visitor);
}
