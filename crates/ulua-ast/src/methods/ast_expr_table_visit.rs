use crate::{
  records::{ast_expr_table::AstExprTable, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprTable, ExprTable, |this, visitor| {
  // item.key 仅 Record 形态非空（其它 kind 为 null，句柄边界折叠跳过，
  // 等价 cpp `if (item.key)` 守卫）；item.value 是表构造必填槽位，parser 总以
  // arena 节点填充。存活与独占前提见 node_handle 模块契约，调用点无 unsafe。
  for item in this.items.iter() {
    if let Some(key) = OptNode::from_ptr(item.key).get_mut() {
      ast_expr_visit_ref(key, visitor);
    }

    if let Some(value) = OptNode::from_ptr(item.value).get_mut() {
      ast_expr_visit_ref(value, visitor);
    }
  }
});
