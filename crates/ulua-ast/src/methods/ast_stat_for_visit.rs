use crate::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_for::AstStatFor, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref, ast_type_visit_ref},
};

impl_visitable!(AstStatFor, StatFor, |this, visitor| {
  // var/from/to/body 已句柄化（Node），step 落可空 OptNode：可变借用沿
  // `&mut self` 逐级传递，dispatch 走 safe 引用形态；annotation 仍为 AstLocal
  // 裸指针槽（records 波次），null 折叠与解引用经 `OptNode` 句柄边界完成
  // （等价 cpp `if (var->annotation)` 守卫，Ast.cpp:814-815）。下钻顺序保 cpp
  // var→from→to→step→body（Ast.cpp:814-823）。
  let var = this.var.get();
  if let Some(ty) = OptNode::from_ptr(var.annotation).get_mut() {
    ast_type_visit_ref(ty, visitor);
  }

  ast_expr_visit_ref(this.from.get_mut(), visitor);
  ast_expr_visit_ref(this.to.get_mut(), visitor);
  if let Some(step) = this.step.get_mut() {
    ast_expr_visit_ref(step, visitor);
  }
  ast_stat_block_visit(this.body.get_mut(), visitor);
});
