use core::ffi::c_void;

use crate::{
  records::{ast_type_pack_variadic::AstTypePackVariadic, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstTypePackVariadic {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_type_pack_variadic(self as *const Self as *mut c_void) {
      unsafe {
        ast_type_visit(self.variadic_type, visitor);
      }
    }
  }
}

pub fn ast_type_pack_variadic_visit<V: AstVisitor + ?Sized>(
  this: &AstTypePackVariadic,
  visitor: &mut V,
) {
  this.visit(visitor);
}
