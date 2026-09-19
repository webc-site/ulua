use core::ffi::c_void;

use crate::{
  records::{ast_expr_interp_string::AstExprInterpString, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprInterpString {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_expr_interp_string(self as *const Self as *mut c_void) {
      for &expr in self.expressions.iter() {
        unsafe {
          ast_expr_visit(expr, visitor);
        }
      }
    }
  }
}
