use core::ffi::c_void;

use crate::{
  records::{ast_expr_group::AstExprGroup, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprGroup {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_expr_group(self as *mut Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
    }
  }
}
