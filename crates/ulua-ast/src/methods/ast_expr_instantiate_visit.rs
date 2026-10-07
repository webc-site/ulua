use crate::{
  functions::visit_type_or_pack_array::visit_type_or_pack_array,
  records::{
    ast_expr_instantiate::AstExprInstantiate, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprInstantiate, ExprInstantiate, |this, visitor| {
  // expr 为 parse 链返回的 arena 存活节点（非 null），与 self 同 arena 且地址
  // 不移动；解引用经 `OptNode` 句柄边界。类型实参数组另行收口。
  if let Some(expr) = OptNode::from_ptr(this.expr).get_mut() {
    ast_expr_visit_ref(expr, visitor);
  }
  visit_type_or_pack_array(visitor, this.type_arguments);
});
