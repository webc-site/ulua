//! Faithful port of Luau `AstType : AstNode` (`Ast/include/Luau/Ast.h`).
//!
//! The abstract base of every type-annotation node. It adds no fields over
//! `AstNode` (only the `as_type` override). `#[repr(C)]` with `base` first keeps
//! the `AstNode` subobject at offset 0 for the RTTI pointer casts in
//! [`crate::rtti`].

use crate::{
  enums::ast_type_ref::AstTypeRef,
  records::{ast_node::AstNode, node_handle::Node},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstType {
  pub base: AstNode,
}

impl AstType {
  /// 将类型注解节点下转为具体引用枚举。
  #[inline]
  pub fn as_type_ref(&self) -> AstTypeRef<'_> {
    AstTypeRef::from_type(self)
  }

  /// 尝试将类型注解节点下转为具体引用枚举。若动态类型不是合法类型注解，返回 `None`。
  #[inline]
  pub fn try_as_type_ref(&self) -> Option<AstTypeRef<'_>> {
    AstTypeRef::try_from_type(self)
  }
}

impl Node<AstType> {
  /// 将类型注解句柄下转为具体引用枚举。
  #[inline]
  pub fn as_type_ref(&self) -> AstTypeRef<'_> {
    self.get().as_type_ref()
  }

  /// 尝试将类型注解句柄下转为具体引用枚举。若动态类型不是合法类型注解，返回 `None`。
  #[inline]
  pub fn try_as_type_ref(&self) -> Option<AstTypeRef<'_>> {
    self.get().try_as_type_ref()
  }
}
