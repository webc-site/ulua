use crate::{
  enums::ast_type_pack_ref::AstTypePackRef,
  records::{ast_node::AstNode, ast_type_pack::AstTypePack},
  rtti::ast_node_as_family,
};

impl AstNode {
  /// cpp `const AstNode::asTypePack() const`：只读判别下转，安全形态
  /// `Option<&AstTypePack>`（收口在 [`ast_node_as_family`]）。
  #[inline]
  pub(crate) fn as_type_pack_const(&self) -> Option<&AstTypePack> {
    ast_node_as_family::<AstTypePack>(self)
  }

  /// 尝试将通用 AST 节点下转为具体类型包引用枚举。若节点不属于 `AstTypePack` 家族，返回 `None`。
  #[inline]
  pub(crate) fn try_as_pack_ref(&self) -> Option<AstTypePackRef<'_>> {
    self
      .as_type_pack_const()
      .and_then(AstTypePack::try_as_pack_ref)
  }

  /// 将通用 AST 节点下转为具体类型包引用枚举。
  ///
  /// # Panics
  /// 若节点不属于 `AstTypePack` 家族，触发 panic。
  #[inline]
  pub fn as_pack_ref(&self) -> AstTypePackRef<'_> {
    self
      .try_as_pack_ref()
      .expect("AstNode class_index 必须为合法的类型包节点类型")
  }
}
