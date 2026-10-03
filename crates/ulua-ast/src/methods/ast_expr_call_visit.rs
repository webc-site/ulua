use crate::{
  records::{
    ast_expr_call::AstExprCall, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprCall, ExprCall, |this, visitor| {
  // func 与 args 元素均为 parser 写入 arena 的存活表达式节点（args 数组同
  // arena 分配）；null 折叠与解引用经 `OptNode` 句柄边界，调用点无 unsafe。
  if let Some(func) = OptNode::from_ptr(this.func).get_mut() {
    ast_expr_visit_ref(func, visitor);
  }

  for &arg in this.args.iter() {
    if let Some(arg) = OptNode::from_ptr(arg).get_mut() {
      ast_expr_visit_ref(arg, visitor);
    }
  }
});
