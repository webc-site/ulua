use core::ffi::c_void;

use crate::{
  records::{ast_expr_function::AstExprFunction, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_stat_visit, ast_type_pack_visit, ast_type_visit},
};

impl AstVisitable for AstExprFunction {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_function(self as *const Self as *mut c_void) {
      for arg_ptr in self.args.iter() {
        unsafe {
          let arg = &**arg_ptr;
          if !arg.annotation.is_null() {
            ast_type_visit(arg.annotation, visitor);
          }
        }
      }

      if !self.vararg_annotation.is_null() {
        unsafe {
          ast_type_pack_visit(self.vararg_annotation, visitor);
        }
      }

      if !self.return_annotation.is_null() {
        unsafe {
          ast_type_pack_visit(self.return_annotation, visitor);
        }
      }

      unsafe {
        ast_stat_visit(self.body as *mut _, visitor);
      }
    }
  }
}

pub fn ast_expr_function_visit(this: &AstExprFunction, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
