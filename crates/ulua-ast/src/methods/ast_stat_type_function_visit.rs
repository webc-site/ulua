use crate::{
  records::{
    ast_stat_type_function::AstStatTypeFunction, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstStatTypeFunction, StatTypeFunction, |this, visitor| {
  // body 是 parse_function_body 产出的 arena 存活 AstExprFunction 指针；静态类型
  // 已知，直接走 `AstVisitable::visit`（cpp `body->visit(v)` 虚分发同一目标），
  // 解引用经 `OptNode` 句柄边界，调用点无 unsafe。
  if let Some(body) = OptNode::from_ptr(this.body).get_mut() {
    body.visit(visitor);
  }
});
