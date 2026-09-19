use core::ffi::c_void;

use crate::{
  functions::visit_type_or_pack_array::visit_type_or_pack_array,
  records::{ast_type_reference::AstTypeReference, ast_visitor::AstVisitor},
  visit::AstVisitable,
};

impl AstVisitable for AstTypeReference {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    if visitor.visit_type_reference(self as *const Self as *mut c_void) {
      visit_type_or_pack_array(visitor, self.parameters);
    }
  }
}
