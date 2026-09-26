//! Faithful port of Luau `AstTypePack : AstNode` (`Ast/include/Luau/Ast.h`).
//!
//! The abstract base of every type-pack node (`AstTypePackExplicit`,
//! `AstTypePackVariadic`, `AstTypePackGeneric`). It adds no fields over
//! `AstNode`. `#[repr(C)]` with `base` first keeps the `AstNode` subobject at
//! offset 0 for the RTTI pointer casts in [`crate::rtti`].

use crate::{
  enums::ast_type_pack_ref::AstTypePackRef,
  records::{ast_node::AstNode, node_handle::Node},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstTypePack {
  pub base: AstNode,
}

impl AstTypePack {
  /// 将类型包节点下转为具体引用枚举。
  #[inline]
  pub fn as_pack_ref(&self) -> AstTypePackRef<'_> {
    AstTypePackRef::from_pack(self)
  }

  /// 尝试将类型包节点下转为具体引用枚举。若动态类型不是合法类型包，返回 `None`。
  #[inline]
  pub fn try_as_pack_ref(&self) -> Option<AstTypePackRef<'_>> {
    AstTypePackRef::try_from_pack(self)
  }
}

impl Node<AstTypePack> {
  /// 将类型包句柄下转为具体引用枚举。
  #[inline]
  pub fn as_pack_ref(&self) -> AstTypePackRef<'_> {
    self.get().as_pack_ref()
  }

  /// 尝试将类型包句柄下转为具体引用枚举。若动态类型不是合法类型包，返回 `None`。
  #[inline]
  pub fn try_as_pack_ref(&self) -> Option<AstTypePackRef<'_>> {
    self.get().try_as_pack_ref()
  }
}
