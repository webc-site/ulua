use core::ffi::c_void;

use crate::{
  records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_stat_visit},
};

impl AstVisitable for AstStatBlock {
  fn visit<V: AstVisitor + ?Sized>(&mut self, visitor: &mut V) {
    if visitor.visit_stat_block(self as *mut Self as *mut c_void) {
      for stat_ptr in self.body.iter() {
        unsafe {
          ast_stat_visit(*stat_ptr, visitor);
        }
      }
    }
  }
}

/// cpp `block->visit(visitor)`，静态类型已是 `AstStatBlock`（无需 class-index
/// 分发）。借用取 `&mut`，与 `crate::visit::AstVisitable::visit` 的 cpp 非 const 语义一致。
pub fn ast_stat_block_visit<V: AstVisitor + ?Sized>(this: &mut AstStatBlock, visitor: &mut V) {
  this.visit(visitor);
}
