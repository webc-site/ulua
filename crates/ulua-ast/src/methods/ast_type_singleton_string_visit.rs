use core::ffi::c_void;

use crate::{
  records::{ast_type_singleton_string::AstTypeSingletonString, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstTypeSingletonString {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    visitor.visit_type_singleton_string(self as *mut Self as *mut c_void);
  }
}
