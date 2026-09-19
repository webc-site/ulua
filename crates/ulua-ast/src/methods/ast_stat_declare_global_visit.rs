use core::ffi::c_void;

use crate::{
  records::{ast_stat_declare_global::AstStatDeclareGlobal, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstStatDeclareGlobal {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_stat_declare_global(self as *mut Self as *mut c_void) {
      unsafe {
        ast_type_visit(self.type_, visitor);
      }
    }
  }
}
