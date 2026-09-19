use core::ffi::c_void;

use crate::{
  records::{ast_expr::AstExpr, ast_stat_function::AstStatFunction, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstStatFunction {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_stat_function(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.name, visitor);
        ast_expr_visit(self.func as *mut AstExpr, visitor);
      }
    }
  }
}
