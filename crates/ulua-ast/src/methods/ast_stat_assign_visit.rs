use core::ffi::c_void;

use crate::{
  records::{ast_stat_assign::AstStatAssign, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstStatAssign {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_assign(self as *const Self as *mut c_void) {
      for &lvalue in self.vars.iter() {
        unsafe {
          ast_expr_visit(lvalue, visitor);
        }
      }

      for &expr in self.values.iter() {
        unsafe {
          ast_expr_visit(expr, visitor);
        }
      }
    }
  }
}

pub fn ast_stat_assign_visit(this: &AstStatAssign, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
