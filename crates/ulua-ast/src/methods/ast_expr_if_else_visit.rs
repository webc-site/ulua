use crate::{
  records::{ast_expr_if_else::AstExprIfElse, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprIfElse, ExprIfElse, |this, visitor| {
  // condition/true_expr/false_expr 已句柄化（node_handle）：`&mut self` 沿借用链
  // 传递独占性，全链路无裸指针；cpp visit 端即三子节点顺序下钻（Ast.cpp:548）。
  ast_expr_visit_ref(this.condition.get_mut(), visitor);
  ast_expr_visit_ref(this.true_expr.get_mut(), visitor);
  ast_expr_visit_ref(this.false_expr.get_mut(), visitor);
});
