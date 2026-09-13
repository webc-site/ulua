use core::ffi::c_void;

use crate::{
  records::{ast_stat_compound_assign::AstStatCompoundAssign, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstStatCompoundAssign {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_compound_assign(self as *const Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.var, visitor);
        ast_expr_visit(self.value, visitor);
      }
    }
  }
}

pub fn ast_stat_compound_assign_visit(this: &AstStatCompoundAssign, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
