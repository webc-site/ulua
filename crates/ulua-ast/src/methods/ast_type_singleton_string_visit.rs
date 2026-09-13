use core::ffi::c_void;

use crate::{
  records::{ast_type_singleton_string::AstTypeSingletonString, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstTypeSingletonString {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_type_singleton_string(self as *const Self as *mut c_void);
  }
}
