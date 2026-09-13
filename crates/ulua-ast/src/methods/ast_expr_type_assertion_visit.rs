use core::ffi::c_void;

use crate::{
  records::{ast_expr_type_assertion::AstExprTypeAssertion, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_type_visit},
};

impl AstVisitable for AstExprTypeAssertion {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_type_assertion(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
        ast_type_visit(self.annotation, visitor);
      }
    }
  }
}
