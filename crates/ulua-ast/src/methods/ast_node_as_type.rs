use core::ptr::NonNull;

use crate::{
  enums::ast_type_ref::AstTypeRef,
  records::{ast_node::AstNode, ast_type::AstType},
  rtti::{ast_node_as_family, ast_node_as_family_mut},
};

impl AstNode {
  /// cpp `AstNode::asType()`：命中 `AstType` 家族则给出类型视图，否则 `None`。
  /// 形态取舍同 [`Self::as_expr`]。
  #[inline]
  pub fn as_type(&mut self) -> Option<NonNull<AstType>> {
    ast_node_as_family_mut::<AstType>(self)
  }

  /// cpp `const AstNode::asType() const`：只读判别下转，安全形态
  /// `Option<&AstType>`（收口在 [`ast_node_as_family`]）。
  #[inline]
  pub fn as_type_const(&self) -> Option<&AstType> {
    ast_node_as_family::<AstType>(self)
  }

  /// 尝试将通用 AST 节点下转为具体类型引用枚举。若节点不属于 `AstType` 家族，返回 `None`。
  #[inline]
  pub(crate) fn try_as_type_ref(&self) -> Option<AstTypeRef<'_>> {
    self.as_type_const().and_then(AstType::try_as_type_ref)
  }

  /// 将通用 AST 节点下转为具体类型引用枚举。
  ///
  /// # Panics
  /// 若节点不属于 `AstType` 家族，触发 panic。
  #[inline]
  pub fn as_type_ref(&self) -> AstTypeRef<'_> {
    self
      .try_as_type_ref()
      .expect("AstNode class_index 必须为合法的类型注解节点类型")
  }
}
