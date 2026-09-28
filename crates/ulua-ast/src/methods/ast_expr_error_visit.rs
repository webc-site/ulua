use crate::{
  records::{ast_expr_error::AstExprError, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstExprError, ExprError, |this, visitor| {
  for &expression in this.expressions.iter() {
    // Safety: ast_expr_visit 接收 arena 中存活节点的裸指针。
    unsafe { ast_expr_visit(expression, visitor) };
  }
});
