use crate::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_while::AstStatWhile, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatWhile, StatWhile, |this, visitor| {
  // condition/body 已句柄化（node_handle::Node）：可变借用沿 `&mut self` 传递，
  // dispatch 走 safe 引用形态，全链路无裸指针。
  ast_expr_visit_ref(this.condition.get_mut(), visitor);
  ast_stat_block_visit(this.body.get_mut(), visitor);
});
