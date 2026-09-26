use crate::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_repeat::AstStatRepeat, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatRepeat, StatRepeat, |this, visitor| {
  // condition/body 已句柄化（node_handle::Node）：可变借用沿 `&mut self` 逐级
  // 传递，dispatch 走 safe 引用形态，全链路无裸指针；cpp visit 端即 body→
  // condition 顺序下钻（Ast.cpp:707/708）。
  ast_stat_block_visit(this.body.get_mut(), visitor);
  ast_expr_visit_ref(this.condition.get_mut(), visitor);
});
