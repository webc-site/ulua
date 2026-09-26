//! Source: `Ast/include/Luau/Ast.h`

use crate::{
  enums::ast_stat_ref::AstStatRef,
  records::{ast_node::AstNode, node_handle::Node},
};

// Ast.h:263 — class AstStat : public AstNode { bool hasSemicolon; }
// Base-class embedding convention: concrete nodes hold `pub base: AstStat`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStat {
  pub base: AstNode,
  pub has_semicolon: bool,
}

impl AstStat {
  /// 将语句节点下转为具体引用枚举。
  #[inline]
  pub fn as_stat_ref(&self) -> AstStatRef<'_> {
    AstStatRef::from_stat(self)
  }

  /// 尝试将语句节点下转为具体引用枚举。若动态类型不是合法语句，返回 `None`。
  #[inline]
  pub fn try_as_stat_ref(&self) -> Option<AstStatRef<'_>> {
    AstStatRef::try_from_stat(self)
  }
}

impl Node<AstStat> {
  /// 将语句句柄下转为具体引用枚举。
  #[inline]
  pub fn as_stat_ref(&self) -> AstStatRef<'_> {
    self.get().as_stat_ref()
  }

  /// 尝试将语句句柄下转为具体引用枚举。若动态类型不是合法语句，返回 `None`。
  #[inline]
  pub fn try_as_stat_ref(&self) -> Option<AstStatRef<'_>> {
    self.get().try_as_stat_ref()
  }
}
