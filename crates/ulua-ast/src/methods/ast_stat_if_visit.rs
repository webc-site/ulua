use core::ffi::c_void;

use crate::{
  records::{ast_stat_if::AstStatIf, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_stat_visit},
};

impl AstVisitable for AstStatIf {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_stat_if(self as *mut Self as *mut c_void) {
      unsafe {
        ast_expr_visit(self.condition, visitor);
        ast_stat_visit(self.thenbody as *mut _, visitor);

        if !self.elsebody.is_null() {
          ast_stat_visit(self.elsebody, visitor);
        }
      }
    }
  }
}
