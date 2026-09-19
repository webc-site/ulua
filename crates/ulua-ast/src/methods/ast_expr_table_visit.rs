use core::ffi::c_void;

use crate::{
  records::{ast_expr_table::AstExprTable, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit},
};

impl AstVisitable for AstExprTable {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_expr_table(self as *mut Self as *mut c_void) {
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
