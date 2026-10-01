use crate::{
  records::{ast_stat_block::AstStatBlock, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_stat_visit_ref},
};

impl_visitable!(AstStatBlock, StatBlock, |this, visitor| {
  // body 已句柄化（node_handle::Nodes）：可变借用沿 `&mut self` 逐级传递，
  // dispatch 走 safe 引用形态，全链路无裸指针。
  for stat in this.body.iter_nodes_mut() {
    ast_stat_visit_ref(stat.get_mut(), visitor);
  }
});

/// cpp `block->visit(visitor)`，静态类型已是 `AstStatBlock`（无需 class-index
/// 分发）。借用取 `&mut`，与 `crate::visit::AstVisitable::visit` 的 cpp 非 const 语义一致。
pub fn ast_stat_block_visit<V: AstVisitor + ?Sized>(this: &mut AstStatBlock, visitor: &mut V) {
  this.visit(visitor);
}
