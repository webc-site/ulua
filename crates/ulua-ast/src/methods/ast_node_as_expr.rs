use core::ptr::NonNull;

use crate::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, ast_node::AstNode},
  rtti::{ast_node_as_family, ast_node_as_family_mut},
};

impl AstNode {
  /// cpp `AstNode::asExpr()`：命中 `AstExpr` 家族则给出表达式视图，否则 `None`。
  /// 判别与下转收口在 [`ast_node_as_family_mut`]（形态取舍见该函数文档）。
  #[inline]
  pub fn as_expr(&mut self) -> Option<NonNull<AstExpr>> {
    ast_node_as_family_mut::<AstExpr>(self)
  }

  /// cpp `const AstNode::asExpr() const`：只读判别下转，共享借用直接给出
  /// `Option<&AstExpr>`（arena 写穿场景请用 [`Self::as_expr`] 的 `NonNull` 形态）。
  #[inline]
  pub fn as_expr_const(&self) -> Option<&AstExpr> {
    ast_node_as_family::<AstExpr>(self)
  }

  /// 将通用 AST 节点下转为具体表达式引用枚举。若节点不属于 `AstExpr` 家族，返回 `None`。
  #[inline]
  pub(crate) fn try_as_expr_ref(&self) -> Option<AstExprRef<'_>> {
    self.as_expr_const().and_then(AstExpr::try_as_expr_ref)
  }

  /// 将通用 AST 节点下转为具体表达式引用枚举。
  ///
  /// # Panics
  /// 若节点不属于 `AstExpr` 家族，触发 panic。
  #[inline]
  pub fn as_expr_ref(&self) -> AstExprRef<'_> {
    self
      .try_as_expr_ref()
      .expect("AstNode class_index 必须为合法的表达式节点类型")
  }
}
