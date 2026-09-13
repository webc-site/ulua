use core::ffi::c_void;

use crate::{
  records::{ast_stat_return::AstStatReturn, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstStatReturn {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_return(self as *const Self as *mut c_void) {
      for &expr in self.list.iter() {
        unsafe {
          ast_expr_visit(expr, visitor);
        }
      }
    }
  }
}
