//! Faithful port of Luau `AstAttr : AstNode` (`Ast/include/Luau/Ast.h`).
//!
//! Hand-ported (the scheduler false-blocks it: its nested `Type` enum resolves
//! by bare name to an unrelated `Type` in `Lexer.h`). The nested
//! `enum Type { Checked, Native, Deprecated, DebugNoinline, Unknown }` is
//! inlined here as `AstAttrType` (matching how the other nested AST enums live
//! in their owner's record file). `DeprecatedInfo` is its own already-translated
//! record; the `deprecatedInfo()`/`visit` methods are separate items.
//!
//! cpp 的 `AstNode::asAttr()` 虚链没有对应导出：基类实现恒返回 nullptr，只有
//! `AstAttr` 自己 override 返回 `this`（Ast.h:186/235），本仓属性判定一律走
//! `has_attribute_in_array`，恒 null 的下转只会误导调用方。

use crate::records::{ast_expr::AstExpr, ast_name::AstName, ast_node::AstNode, node_handle::Nodes};

/// C++ `AstAttr::Type`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstAttrType {
  Checked,
  Native,
  Deprecated,
  DebugNoinline,
  Unknown,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstAttr {
  pub base: AstNode,
  pub r#type: AstAttrType,
  /// cpp `AstArray<AstExpr*> args`（Ast.h:245）：元素出自 parse_call_list 的 arena
  /// 分配（恒非空），空数组即 `Nodes::empty()`（cpp `{nullptr, 0}` 哨兵消失）。
  pub args: Nodes<AstExpr>,
  pub name: AstName,
}
