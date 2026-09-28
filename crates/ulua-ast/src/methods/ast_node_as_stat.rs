use crate::{
  enums::ast_stat_ref::AstStatRef,
  records::{ast_node::AstNode, ast_stat::AstStat},
  rtti::ast_node_as_family,
};

impl AstNode {
  /// cpp `const AstNode::asStat() const`：只读判别下转，安全形态
  /// `Option<&AstStat>`（收口在 [`ast_node_as_family`]）。
  #[inline]
  pub fn as_stat_const(&self) -> Option<&AstStat> {
    ast_node_as_family::<AstStat>(self)
  }

  /// 尝试将通用 AST 节点下转为具体语句引用枚举。若节点不属于 `AstStat` 家族，返回 `None`。
  #[inline]
  pub(crate) fn try_as_stat_ref(&self) -> Option<AstStatRef<'_>> {
    self.as_stat_const().and_then(AstStat::try_as_stat_ref)
  }

  /// 将通用 AST 节点下转为具体语句引用枚举。
  ///
  /// # Panics
  /// 若节点不属于 `AstStat` 家族，触发 panic。
  #[inline]
  pub fn as_stat_ref(&self) -> AstStatRef<'_> {
    self
      .try_as_stat_ref()
      .expect("AstNode class_index 必须为合法的语句节点类型")
  }
}
