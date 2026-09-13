use core::ffi::c_void;

use crate::{
  records::{ast_type_singleton_bool::AstTypeSingletonBool, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstTypeSingletonBool {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_type_singleton_bool(self as *const Self as *mut c_void);
  }
}
