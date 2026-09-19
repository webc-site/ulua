use core::ffi::c_void;

use crate::{
  records::{ast_type_union::AstTypeUnion, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstTypeUnion {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_type_union(self as *mut Self as *mut c_void) {
      for &type_ptr in self.types.iter() {
        unsafe {
          ast_type_visit(type_ptr, visitor);
        }
      }
    }
  }
}
