use core::ffi::c_void;

use crate::{
  records::{ast_generic_type_pack::AstGenericTypePack, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_pack_visit},
};

impl AstVisitable for AstGenericTypePack {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_generic_type_pack(self as *const Self as *mut c_void)
      && !self.default_value.is_null()
    {
      unsafe {
        ast_type_pack_visit(self.default_value, visitor);
      }
    }
  }
}

pub fn ast_generic_type_pack_visit<V: AstVisitor + ?Sized>(
  this: &AstGenericTypePack,
  visitor: &mut V,
) {
  this.visit(visitor);
}
