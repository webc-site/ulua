use core::ffi::c_void;

use crate::{
  records::{ast_stat_for_in::AstStatForIn, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_expr_visit, ast_stat_visit, ast_type_visit},
};

impl AstVisitable for AstStatForIn {
  fn visit(&self, visitor: &mut dyn AstVisitor) {
    if visitor.visit_stat_for_in(self as *const Self as *mut c_void) {
      for &var in self.vars.iter() {
        // SAFETY: var 指向 arena 中存活的 AstLocal 节点。
        unsafe {
          if !var.is_null() && !(*var).annotation.is_null() {
            ast_type_visit((*var).annotation, visitor);
          }
        }
      }

      for &expr in self.values.iter() {
        // SAFETY: ast_expr_visit 接收 arena 中存活节点的裸指针。
        unsafe { ast_expr_visit(expr, visitor) };
      }

      // SAFETY: ast_stat_visit 接收 arena 中存活节点的裸指针。
      unsafe {
        ast_stat_visit(self.body as *mut _, visitor);
      }
    }
  }
}

pub fn ast_stat_for_in_visit(this: &AstStatForIn, visitor: &mut dyn AstVisitor) {
  this.visit(visitor);
}
