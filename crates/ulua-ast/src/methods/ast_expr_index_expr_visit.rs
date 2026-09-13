use core::ffi::c_void;

use crate::{
  records::{ast_expr_index_expr::AstExprIndexExpr, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprIndexExpr {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_index_expr(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.expr, visitor);
        ast_expr_visit(self.index, visitor);
      }
    }
  }
}

pub fn ast_expr_index_expr_visit(this: &AstExprIndexExpr, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
