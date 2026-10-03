use crate::{
  records::{
    ast_expr_interp_string::AstExprInterpString, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstExprInterpString, ExprInterpString, |this, visitor| {
  // expressions 元素由 parse_string_with_interpolation 逐个以 arena 节点填充；
  // null 折叠与解引用经 `OptNode` 句柄边界，调用点无 unsafe。
  for &expr in this.expressions.iter() {
    if let Some(expr) = OptNode::from_ptr(expr).get_mut() {
      ast_expr_visit_ref(expr, visitor);
    }
  }
});
