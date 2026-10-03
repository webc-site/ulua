use crate::{
  records::{
    ast_stat_return::AstStatReturn, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatReturn, StatReturn, |this, visitor| {
  // list 元素是 parse_expr_list 写入 arena 的返回值表达式节点（可含 null 槽位，
  // 句柄边界折叠跳过）；地址稳定，单线程独占遍历。调用点无 unsafe。
  for &expr in this.list.iter() {
    if let Some(expr) = OptNode::from_ptr(expr).get_mut() {
      ast_expr_visit_ref(expr, visitor);
    }
  }
});
