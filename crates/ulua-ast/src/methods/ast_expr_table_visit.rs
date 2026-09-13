use core::ffi::c_void;

use crate::{
  records::{ast_expr_table::AstExprTable, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprTable {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_expr_table(self as *const Self as *mut c_void) {
      for item in self.items.iter() {
        if !item.key.is_null() {
          unsafe {
            ast_expr_visit(item.key, visitor);
          }
        }

        unsafe {
          ast_expr_visit(item.value, visitor);
        }
      }
    }
  }
}

pub fn ast_expr_table_visit(this: &AstExprTable, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
