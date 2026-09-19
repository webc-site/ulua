use core::ffi::c_void;

use crate::{
  records::{ast_attr::AstAttr, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstAttr {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    visitor.visit_attr(self as *const Self as *mut c_void);
  }
}
