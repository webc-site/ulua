use core::ffi::c_void;

use crate::{
  records::{
    ast_expr::AstExpr, ast_stat_type_function::AstStatTypeFunction, ast_visitor::AstVisitor,
  },
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstStatTypeFunction {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_type_function(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.body as *mut AstExpr, visitor);
      }
    }
  }
}

pub fn ast_stat_type_function_visit(this: &AstStatTypeFunction, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
