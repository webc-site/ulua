use core::ffi::c_void;

use crate::{
  records::{ast_stat_local::AstStatLocal, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_type_visit},
};

impl AstVisitable for AstStatLocal {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_local(self as *const Self as *mut c_void) {
      for var_ptr in self.vars.iter() {
        let var = unsafe { &**var_ptr };
        if !var.annotation.is_null() {
          unsafe {
            ast_type_visit(var.annotation, visitor);
          }
        }
      }

      for expr_ptr in self.values.iter() {
        unsafe {
          ast_expr_visit(*expr_ptr, visitor);
        }
      }
    }
  }
}

pub fn ast_stat_local_visit(this: &AstStatLocal, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
