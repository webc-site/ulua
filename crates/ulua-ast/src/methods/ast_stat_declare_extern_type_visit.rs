use core::ffi::c_void;

use crate::{
  records::{ast_stat_declare_extern_type::AstStatDeclareExternType, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_visit},
};

impl AstVisitable for AstStatDeclareExternType {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_stat_declare_extern_type(self as *mut Self as *mut c_void) {
      for prop in self.props.iter() {
        unsafe {
          ast_type_visit(prop.ty, visitor);
        }
      }
    }
  }
}
