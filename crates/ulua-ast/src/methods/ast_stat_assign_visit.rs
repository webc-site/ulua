use crate::{
  records::{
    ast_stat_assign::AstStatAssign, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatAssign, StatAssign, |this, visitor| {
  // vars 元素是 parser 校验过的左值表达式节点、values 元素是 parse_expr_list
  // 写入 arena 的存活表达式节点（地址稳定）；null 折叠与解引用统一经
  // `OptNode` 句柄边界（等价旧 dispatch 短路），调用点无 unsafe。
  for &lvalue in this.vars.iter() {
    if let Some(lv) = OptNode::from_ptr(lvalue).get_mut() {
      ast_expr_visit_ref(lv, visitor);
    }
  }

  for &expr in this.values.iter() {
    if let Some(expr) = OptNode::from_ptr(expr).get_mut() {
      ast_expr_visit_ref(expr, visitor);
    }
  }
});
