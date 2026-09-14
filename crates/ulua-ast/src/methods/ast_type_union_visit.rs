use core::ffi::c_void;

use crate::{
  records::{ast_type_union::AstTypeUnion, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstTypeUnion {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_type_union(self as *const Self as *mut c_void) {
      for &type_ptr in self.types.iter() {
        unsafe {
          ast_type_visit(type_ptr, visitor);
        }
      }
    }
  }
}

pub fn ast_type_union_visit<V: AstVisitor + ?Sized>(this: &AstTypeUnion, visitor: &mut V) {
  this.visit(visitor);
}
