use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_group::AstExprGroup, ast_expr_type_assertion::AstExprTypeAssertion,
  },
  rtti::{AstNodeClass, ast_node_try_as},
};

use crate::records::node::Node;

/// C++ `unwrapExprOfType<T>`：剥开 group / type assertion 包装层，
/// 返回首个动态类型为 T 的节点句柄。入参/返回值均为地址句柄（只作键位/
/// 继续下钻，节点存活契约见 `Node::borrow`），无裸指针解引用。
pub(crate) fn unwrap_expr_of_type<T>(mut node: Node<AstExpr>) -> Option<Node<T>>
where
  T: AstNodeClass + 'static,
{
  loop {
    let base = &node.borrow().base;

    if let Some(expr) = ast_node_try_as::<T>(base) {
      return Some(expr.into());
    }

    if let Some(group) = ast_node_try_as::<AstExprGroup>(base) {
      node = group.expr.into();
    } else {
      let assertion = ast_node_try_as::<AstExprTypeAssertion>(base)?;
      node = assertion.expr.into();
    }
  }
}
