use core::ffi::c_void;

use crate::{
  records::{ast_type_typeof::AstTypeTypeof, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstTypeTypeof {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_type_typeof(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
      }
    }
  }
}

pub fn ast_type_typeof_visit(this: &AstTypeTypeof, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
