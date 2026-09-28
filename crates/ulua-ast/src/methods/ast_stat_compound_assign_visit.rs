use crate::{
  records::{ast_stat_compound_assign::AstStatCompoundAssign, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(
  AstStatCompoundAssign,
  StatCompoundAssign,
  |this, visitor| {
    // var/value 已句柄化（node_handle::Node）：可变借用沿 `&mut self` 逐级传递，
    // dispatch 走 safe 引用形态，全链路无裸指针；cpp visit 端即二子节点顺序下钻
    // （Ast.cpp:896/897）。
    ast_expr_visit_ref(this.var.get_mut(), visitor);
    ast_expr_visit_ref(this.value.get_mut(), visitor);
  }
);
