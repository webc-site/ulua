//! Faithful port of Luau `AstNode` — the base of every AST node
//! (`Ast/include/Luau/Ast.h`).
//!
//! C++ `AstNode` is `{ const int classIndex; Location location; }` plus a vtable
//! (`visit`, `as_expr`/`as_stat`/`as_type`, and the `is<T>()`/`as<T>()`
//! RTTI helpers). Luau nodes are standard-layout single-inheritance, so a
//! `class X : Y` lays `Y` out at offset 0. We reproduce that with `#[repr(C)]`
//! and a `pub base: Y` first field on every node, which makes the C++
//! `static_cast<T*>(this)` downcast a plain pointer cast (see [`crate::rtti`]).
//!
//! `classIndex` is `const` in C++ (set once at construction); Rust has no const
//! fields, so it is a plain `pub i32` written by each node's constructor to that
//! node's [`crate::rtti::AstNodeClass::CLASS_INDEX`]. `visit` and the
//! `as{Expr,Stat,Type}` virtuals are separate method items. cpp 的第四个虚函数
//! `asAttr()` 不导出，理由见 [`crate::records::ast_attr`] 的模块注释。

use crate::records::location::Location;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstNode {
  pub class_index: i32,
  pub location: Location,
}
