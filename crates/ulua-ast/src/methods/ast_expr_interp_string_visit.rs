use core::ffi::c_void;

use crate::{
  records::{ast_expr_interp_string::AstExprInterpString, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprInterpString {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_interp_string(self as *const Self as *mut c_void) {
      for &expr in self.expressions.iter() {
        unsafe {
          ast_expr_visit(expr, visitor);
        }
      }
    }
  }
}

pub fn ast_expr_interp_string_visit(this: &AstExprInterpString, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
