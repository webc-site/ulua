use core::ffi::c_void;

use crate::{
  records::{ast_attr::AstAttr, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstAttr {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_attr(self as *const Self as *mut c_void);
  }
}

pub fn ast_attr_visit(this: &AstAttr, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
