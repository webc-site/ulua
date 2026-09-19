use core::ffi::c_void;

use crate::{
  records::{ast_generic_type::AstGenericType, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstGenericType {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_generic_type(self as *mut Self as *mut c_void)
      && !self.default_value.is_null()
    {
      unsafe {
        ast_type_visit(self.default_value, visitor);
      }
    }
  }
}
