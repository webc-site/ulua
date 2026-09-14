use core::ffi::c_void;

use crate::{
  records::{ast_stat_repeat::AstStatRepeat, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_stat_visit},
};

impl AstVisitable for AstStatRepeat {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_stat_repeat(self as *const Self as *mut c_void) {
      unsafe {
        ast_stat_visit(self.body as *mut _, visitor);
        ast_expr_visit(self.condition, visitor);
      }
    }
  }
}

pub fn ast_stat_repeat_visit<V: AstVisitor + ?Sized>(this: &AstStatRepeat, visitor: &mut V) {
  this.visit(visitor);
}
