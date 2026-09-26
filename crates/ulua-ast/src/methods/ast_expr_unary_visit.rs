use crate::{
  records::{ast_expr_unary::AstExprUnary, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprUnary, ExprUnary, |this, visitor| {
  // expr 已句柄化（node_handle）：`&mut self` 沿借用链传递独占性，全链路无裸指针。
  ast_expr_visit_ref(this.expr.get_mut(), visitor);
});
