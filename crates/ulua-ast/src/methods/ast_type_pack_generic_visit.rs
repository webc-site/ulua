use core::ffi::c_void;

use crate::{
  records::{ast_type_pack_generic::AstTypePackGeneric, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstTypePackGeneric {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    visitor.visit_type_pack_generic(self as *const Self as *mut c_void);
  }
}
