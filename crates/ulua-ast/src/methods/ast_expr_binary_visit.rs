use crate::{
  records::{ast_expr_binary::AstExprBinary, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprBinary, ExprBinary, |this, visitor| {
  // left/right 已句柄化（node_handle）：`&mut self` 沿借用链传递独占性，全链路无裸指针。
  ast_expr_visit_ref(this.left.get_mut(), visitor);
  ast_expr_visit_ref(this.right.get_mut(), visitor);
});
