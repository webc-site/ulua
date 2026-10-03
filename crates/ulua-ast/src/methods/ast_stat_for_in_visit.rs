use crate::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_stat_for_in::AstStatForIn, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref, ast_type_visit_ref},
};

impl_visitable!(AstStatForIn, StatForIn, |this, visitor| {
  // vars/values 元素槽仍为裸指针（records 波次）：null 折叠与解引用统一经
  // `OptNode` 句柄边界（存活/独占契约见 node_handle 模块头），调用点无 `unsafe`。
  for &var_ptr in this.vars.iter() {
    if let Some(var) = OptNode::from_ptr(var_ptr).get() {
      if let Some(ty) = OptNode::from_ptr(var.annotation).get_mut() {
        ast_type_visit_ref(ty, visitor);
      }
    }
  }

  for &expr in this.values.iter() {
    if let Some(e) = OptNode::from_ptr(expr).get_mut() {
      ast_expr_visit_ref(e, visitor);
    }
  }

  // body 已句柄化为 Node：可变借用沿 &mut self 传递，dispatch 走 safe 引用
  // 形态（cpp Ast.cpp:861 无守卫下钻同序）。
  ast_stat_block_visit(this.body.get_mut(), visitor);
});
