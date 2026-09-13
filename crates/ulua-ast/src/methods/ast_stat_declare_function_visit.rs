use core::ffi::c_void;

use crate::{
  functions::visit_type_list::visit_type_list,
  records::{ast_stat_declare_function::AstStatDeclareFunction, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_pack_visit},
};

impl AstVisitable for AstStatDeclareFunction {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_declare_function(self as *const Self as *mut c_void) {
      unsafe {
        visit_type_list(visitor, &self.params);

        if !self.ret_types.is_null() {
          ast_type_pack_visit(self.ret_types, visitor);
        }
      }
    }
  }
}
