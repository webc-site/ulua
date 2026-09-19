use core::ffi::c_void;

use crate::{
  records::{
    ast_expr::AstExpr, ast_stat_local_function::AstStatLocalFunction, ast_visitor::AstVisitor,
  },
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstStatLocalFunction {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_stat_local_function(self as *mut Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.func as *mut AstExpr, visitor);
      }
    }
  }
}
