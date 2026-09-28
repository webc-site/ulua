use crate::{
  records::{ast_stat_expr::AstStatExpr, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatExpr, StatExpr, |this, visitor| {
  // expr 已句柄化（node_handle::Node）：可变借用沿 `&mut self` 传递，dispatch
  // 走 safe 引用形态，全链路无裸指针；cpp visit 端即单子节点下钻（Ast.cpp:756）。
  ast_expr_visit_ref(this.expr.get_mut(), visitor);
});
