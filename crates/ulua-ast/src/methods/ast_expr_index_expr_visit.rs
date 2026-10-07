use crate::{
  records::{ast_expr_index_expr::AstExprIndexExpr, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprIndexExpr, ExprIndexExpr, |this, visitor| {
  // expr/index 已句柄化（node_handle）：`&mut self` 沿借用链传递独占性，全链路无裸指针。
  ast_expr_visit_ref(this.expr.get_mut(), visitor);
  ast_expr_visit_ref(this.index.get_mut(), visitor);
});
