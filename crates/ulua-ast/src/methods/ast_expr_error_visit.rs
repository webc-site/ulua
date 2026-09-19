use core::ffi::c_void;

use crate::{
  records::{ast_expr_error::AstExprError, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprError {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_expr_error(self as *mut Self as *mut c_void) {
      for &expression in self.expressions.iter() {
        // SAFETY: ast_expr_visit 接收 arena 中存活节点的裸指针。
        unsafe { ast_expr_visit(expression, visitor) };
      }
    }
  }
}
