use core::ffi::c_void;

use crate::{
  functions::visit_type_list::visit_type_list,
  records::{ast_type_function::AstTypeFunction, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_type_pack_visit},
};

impl AstVisitable for AstTypeFunction {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_type_function(self as *const Self as *mut c_void) {
      unsafe {
        visit_type_list(visitor, &self.arg_types);

        if !self.return_types.is_null() {
          ast_type_pack_visit(self.return_types, visitor);
        }
      }
    }
  }
}

pub fn ast_type_function_visit(this: &AstTypeFunction, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
