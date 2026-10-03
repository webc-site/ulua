use crate::{
  records::{ast_expr_error::AstExprError, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprError, ExprError, |this, visitor| {
  // expressions 元素为 arena 存活节点或 null（句柄边界折叠跳过），调用点无 unsafe。
  for &expression in this.expressions.iter() {
    if let Some(expr) = OptNode::from_ptr(expression).get_mut() {
      ast_expr_visit_ref(expr, visitor);
    }
  }
});
