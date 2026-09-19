use core::ffi::c_void;

use crate::{
  records::{ast_type_group::AstTypeGroup, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstTypeGroup {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_type_group(self as *const Self as *mut c_void) {
      unsafe {
        ast_type_visit(self.type_, visitor);
      }
    }
  }
}
