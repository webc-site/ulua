use core::ffi::c_void;

use crate::{
  records::{ast_expr_index_name::AstExprIndexName, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprIndexName {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_index_name(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
    }
  }
}
