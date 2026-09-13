use core::ffi::c_void;

use crate::{
  records::{ast_stat_declare_global::AstStatDeclareGlobal, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstStatDeclareGlobal {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_declare_global(self as *const Self as *mut c_void) {
      unsafe {
        ast_type_visit(self.type_, visitor);
      }
    }
  }
}
