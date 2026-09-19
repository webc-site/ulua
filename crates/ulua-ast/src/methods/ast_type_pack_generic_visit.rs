use core::ffi::c_void;

use crate::{
  records::{ast_type_pack_generic::AstTypePackGeneric, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstTypePackGeneric {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    visitor.visit_type_pack_generic(self as *mut Self as *mut c_void);
  }
}
