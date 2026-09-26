//! Faithful port of Luau `AstExpr : AstNode` (`Ast/include/Luau/Ast.h`).
//!
//! The abstract base of every expression node. It adds no fields over `AstNode`
//! (only the `as_expr` override). `#[repr(C)]` with `base` first keeps the
//! `AstNode` subobject at offset 0 so the RTTI pointer casts in [`crate::rtti`]
//! are sound.

use crate::{enums::ast_expr_ref::AstExprRef, records::ast_node::AstNode};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExpr {
  pub base: AstNode,
}

impl AstExpr {
  /// 将表达式节点下转为具体引用枚举。
  #[inline]
  pub fn as_expr_ref(&self) -> AstExprRef<'_> {
    AstExprRef::from_expr(self)
  }
}

impl crate::records::node_handle::Node<AstExpr> {
  /// 将表达式句柄下转为具体引用枚举。
  #[inline]
  pub fn as_expr_ref(&self) -> AstExprRef<'_> {
    self.get().as_expr_ref()
  }

  /// 尝试将表达式句柄下转为具体引用枚举。若动态类型不是合法表达式，返回 `None`。
  #[inline]
  pub fn try_as_expr_ref(&self) -> Option<AstExprRef<'_>> {
    self.get().try_as_expr_ref()
  }
}
