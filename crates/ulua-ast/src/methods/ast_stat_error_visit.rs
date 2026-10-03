use crate::{
  records::{
    ast_stat_error::AstStatError, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref, ast_stat_visit_ref},
};

impl_visitable!(AstStatError, StatError, |this, visitor| {
  // expressions/statements 元素来自错误恢复路径以 arena 节点填充的槽位，
  // null 由句柄边界折叠跳过；地址稳定、单线程独占遍历，调用点无 unsafe。
  for &expression in this.expressions.iter() {
    if let Some(expr) = OptNode::from_ptr(expression).get_mut() {
      ast_expr_visit_ref(expr, visitor);
    }
  }

  for &statement in this.statements.iter() {
    if let Some(stat) = OptNode::from_ptr(statement).get_mut() {
      ast_stat_visit_ref(stat, visitor);
    }
  }
});
