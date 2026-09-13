use core::ffi::c_void;

use crate::{
  records::{ast_type_pack_explicit::AstTypePackExplicit, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_pack_visit, ast_type_visit},
};

impl AstVisitable for AstTypePackExplicit {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_type_pack_explicit(self as *const Self as *mut c_void) {
      for &type_ptr in self.type_list.types.iter() {
        unsafe {
          ast_type_visit(type_ptr, visitor);
        }
      }

      if !self.type_list.tail_type.is_null() {
        unsafe {
          ast_type_pack_visit(self.type_list.tail_type, visitor);
        }
      }
    }
  }
}

pub fn ast_type_pack_explicit_visit(this: &AstTypePackExplicit, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
