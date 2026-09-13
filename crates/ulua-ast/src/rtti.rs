//! AST RTTI mechanism — the faithful Rust analog of Luau's
//! `AstRtti<T>::value` / `LUAU_RTTI(Class)` / `AstNode::is<T>()` / `as<T>()`.
//! Reference: `luau/Ast/include/Luau/Ast.h` (the `AstNode` base + the
//! `LUAU_RTTI` macro).
//!
//! In C++ every node carries a `const int classIndex` set at construction to a
//! per-type id, and `node->as<T>()` is `classIndex == T::ClassIndex() ?
//! static_cast<T*>(this) : nullptr`. The cast is sound because Luau nodes are
//! standard-layout single-inheritance, so the base subobject sits at offset 0.
//!
//! We reproduce that exactly: every node is `#[repr(C)]` with its parent as the
//! first field (`pub base: Parent`), so a `*mut AstNode` that actually points at
//! an `AstExprGroup` can be reinterpreted as `*mut AstExprGroup` once the class
//! index matches. The class index is a compile-time hash of the type name, so
//! each node file is self-contained (no shared mutable counter / central
//! registry to serialize against, unlike C++'s `++gAstRttiIndex`). The exact
//! integer is irrelevant — only that it is unique per type and stable — which
//! the `rtti_indices_unique` test enforces over the full node set.

use core::ptr::{null, null_mut};

use crate::records::{
  ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_type::AstType,
  ast_type_pack::AstTypePack, cst_node::CstNode,
};

/// Stable per-type class index, the analog of `AstRtti<Class>::value`. FNV-1a
/// over the class name, folded to a positive `i32` so it can never collide with
/// a future "no class" sentinel (e.g. `-1`).
pub const fn ast_rtti_index(name: &str) -> i32 {
  let bytes = name.as_bytes();
  let mut hash: u32 = 0x811c_9dc5;
  let mut i = 0;
  while i < bytes.len() {
    hash ^= bytes[i] as u32;
    hash = hash.wrapping_mul(0x0100_0193);
    i += 1;
  }
  (hash & 0x7fff_ffff) as i32
}

/// Implemented by every concrete AST node type — the analog of the
/// `LUAU_RTTI(Class)` macro, which expands to `static int ClassIndex()`.
///
/// A node `class X : Y` becomes `#[repr(C)] struct X { pub base: Y, ... }` plus
/// `impl AstNodeClass for X { const CLASS_INDEX: i32 = ast_rtti_index("X"); }`.
pub trait AstNodeClass {
  /// The node's RTTI id; mirrors `T::ClassIndex()`.
  const CLASS_INDEX: i32;
}

/// `node->is<T>()` — does this node have `T`'s dynamic type?
///
/// # Safety
/// Implementing types that are raw pointers must safely handle null or be valid to dereference.
pub unsafe trait AstNodeRef {
  /// Returns the dynamic class index of this AST node, or `None` if null.
  ///
  /// # Safety
  /// If `self` is a raw pointer, it must be either null or valid for dereference.
  unsafe fn class_index(self) -> Option<i32>;
}

unsafe impl AstNodeRef for &AstNode {
  #[inline]
  unsafe fn class_index(self) -> Option<i32> {
    Some(self.class_index)
  }
}

unsafe impl AstNodeRef for *mut AstNode {
  #[inline]
  unsafe fn class_index(self) -> Option<i32> {
    if self.is_null() {
      None
    } else {
      unsafe { Some((*self).class_index) }
    }
  }
}

unsafe impl AstNodeRef for *const AstNode {
  #[inline]
  unsafe fn class_index(self) -> Option<i32> {
    if self.is_null() {
      None
    } else {
      unsafe { Some((*self).class_index) }
    }
  }
}

unsafe impl AstNodeRef for &AstType {
  #[inline]
  unsafe fn class_index(self) -> Option<i32> {
    Some(self.base.class_index)
  }
}

unsafe impl AstNodeRef for &AstExpr {
  #[inline]
  unsafe fn class_index(self) -> Option<i32> {
    Some(self.base.class_index)
  }
}

unsafe impl AstNodeRef for &AstStat {
  #[inline]
  unsafe fn class_index(self) -> Option<i32> {
    Some(self.base.class_index)
  }
}

unsafe impl AstNodeRef for &AstTypePack {
  #[inline]
  unsafe fn class_index(self) -> Option<i32> {
    Some(self.base.class_index)
  }
}

#[inline]
pub fn ast_node_is<T: AstNodeClass>(node: impl AstNodeRef) -> bool {
  unsafe { node.class_index() == Some(T::CLASS_INDEX) }
}

/// `node->as<T>()` — downcast a base-node pointer to `*mut T`, or null when the
/// dynamic type does not match.
///
/// # Safety
/// `node` must be null or point to a live node whose first field is (transitively)
/// an `AstNode` — i.e. any of the generated `#[repr(C)]` node structs. This is the
/// same precondition as the C++ `static_cast<T*>(this)` it replaces.
#[inline]
pub unsafe fn ast_node_as<T: AstNodeClass>(node: *mut AstNode) -> *mut T {
  // SAFETY: 前置条件见函数级 # Safety；class_index 命中后 repr(C)
  // 单继承布局保证 cast::<T> 与基址重合。
  unsafe {
    if !node.is_null() && (*node).class_index == T::CLASS_INDEX {
      node.cast::<T>()
    } else {
      null_mut()
    }
  }
}

/// `const` variant of [`ast_node_as`] for `*const AstNode`.
///
/// # Safety
/// Same precondition as [`ast_node_as`].
#[inline]
pub unsafe fn ast_node_as_const<T: AstNodeClass>(node: *const AstNode) -> *const T {
  unsafe {
    if !node.is_null() && (*node).class_index == T::CLASS_INDEX {
      node.cast::<T>()
    } else {
      null()
    }
  }
}

/// safe 版 [`ast_node_as`]：引用进、`Option<&T>` 出，null 检查与裸指针一并消失。
/// 这是 Rust 侧下转的惯用形态，unsafe 收口在本函数一行的 cast 里。
#[inline]
pub fn ast_node_try_as<T: AstNodeClass>(node: &AstNode) -> Option<&T> {
  if node.class_index == T::CLASS_INDEX {
    // SAFETY: 动态类型已由 class_index 判定，#[repr(C)] 单继承保证 T 的首字段
    // 即 AstNode，cast 布局有效；引用保证非空与存活。
    Some(unsafe { &*(node as *const AstNode).cast::<T>() })
  } else {
    None
  }
}

/// `ast_node_try_as_mut::<T>(node)` 的合并形态：裸指针进、`Option<&mut T>` 出，
/// 把 as + as_mut 两次 unsafe 收敛为一次。
///
/// # Safety
/// `node` 必须为 null 或指向 arena 中存活的 repr(C) 节点（同 [`ast_node_as`]）。
#[inline]
pub unsafe fn ast_node_try_as_mut<T: AstNodeClass>(node: *mut AstNode) -> Option<&'static mut T> {
  // SAFETY: 前置条件见函数级 # Safety；class_index 命中后 cast 布局有效。
  unsafe { ast_node_as::<T>(node).as_mut() }
}

/// `&Derived → &AstNode` 上转，C++ 隐式基类转换的对应物。
/// 所有 AST 节点为 `#[repr(C)]` 且 `base` 为首字段，基类指针与派生指针值相等。
#[inline]
pub fn as_node<T: AstNodeClass>(node: &T) -> &AstNode {
  // SAFETY: repr(C) 单继承布局，T 的首字段传递性地是 AstNode，指针值相等。
  unsafe { &*(&raw const *node).cast::<AstNode>() }
}

/// CST spelling of [`ast_rtti_index`] (CST and AST share the index function but
/// separate index spaces). Provided so `LUAU_CST_RTTI(Class)` translations can
/// read naturally as `cst_rtti_index("CstX")`.
#[inline]
pub const fn cst_rtti_index(name: &str) -> i32 {
  ast_rtti_index(name)
}

/// CST analog of [`AstNodeClass`] — the `LUAU_CST_RTTI(Class)` macro, which
/// expands to `static int CstClassIndex()`. CST nodes form a separate RTTI
/// space (`gCstRttiIndex`) and a `CstNode*` is never cross-cast to an `AstNode*`,
/// so reusing [`ast_rtti_index`] for the index value is sound — uniqueness only
/// has to hold among CST names ([`tests::cst_rtti_indices_unique`]).
pub trait CstNodeClass {
  /// The node's CST RTTI id; mirrors `T::CstClassIndex()`.
  const CLASS_INDEX: i32;
}

/// `cstNode->is<T>()`.
#[inline]
pub fn cst_node_is<T: CstNodeClass>(node: &CstNode) -> bool {
  node.class_index == T::CLASS_INDEX
}

/// `cstNode->as<T>()` — downcast a base `*mut CstNode` to `*mut T`, or null on
/// mismatch.
///
/// # Safety
/// `node` must be null or point to a live CST node whose first field is
/// (transitively) a `CstNode` — i.e. any generated `#[repr(C)]` CST node struct.
#[inline]
pub unsafe fn cst_node_as<T: CstNodeClass>(node: *mut CstNode) -> *mut T {
  unsafe {
    if !node.is_null() && (*node).class_index == T::CLASS_INDEX {
      node.cast::<T>()
    } else {
      null_mut()
    }
  }
}

#[cfg(test)]
mod tests {
  use alloc::{collections::BTreeMap, vec::Vec};

  use super::ast_rtti_index;

  /// 对一组类名计算 RTTI 索引并收集碰撞（名字对 + 相同索引）。
  fn collect_collisions(names: &[&'static str]) -> Vec<(&'static str, &'static str, i32)> {
    let mut seen: BTreeMap<i32, &str> = BTreeMap::new();
    let mut collisions = Vec::new();
    for &name in names {
      let idx = ast_rtti_index(name);
      if let Some(&prev) = seen.get(&idx) {
        collisions.push((prev, name, idx));
      } else {
        seen.insert(idx, name);
      }
    }
    collisions
  }

  /// The full set of `LUAU_RTTI(Class)` names in `Ast.h` / `Cst.h`. The C++
  /// guarantees uniqueness by construction (`++gAstRttiIndex`); our hash-based
  /// scheme must be checked. If a future node collides, add a salt to its name
  /// here and in its `impl AstNodeClass`.
  const RTTI_NAMES: &[&str] = &[
    "AstAttr",
    "AstGenericType",
    "AstGenericTypePack",
    "AstExprGroup",
    "AstExprConstantNil",
    "AstExprConstantBool",
    "AstExprConstantNumber",
    "AstExprConstantString",
    "AstExprLocal",
    "AstExprGlobal",
    "AstExprVarargs",
    "AstExprCall",
    "AstExprIndexName",
    "AstExprIndexExpr",
    "AstExprFunction",
    "AstExprTable",
    "AstExprUnary",
    "AstExprBinary",
    "AstExprTypeAssertion",
    "AstExprIfElse",
    "AstExprInterpString",
    "AstExprError",
    "AstStatBlock",
    "AstStatIf",
    "AstStatWhile",
    "AstStatRepeat",
    "AstStatBreak",
    "AstStatContinue",
    "AstStatReturn",
    "AstStatExpr",
    "AstStatLocal",
    "AstStatFor",
    "AstStatForIn",
    "AstStatAssign",
    "AstStatCompoundAssign",
    "AstStatFunction",
    "AstStatLocalFunction",
    "AstStatTypeAlias",
    "AstStatTypeFunction",
    "AstStatDeclareGlobal",
    "AstStatDeclareFunction",
    "AstStatDeclareExternType",
    "AstStatError",
    "AstTypeReference",
    "AstTypeTable",
    "AstTypeFunction",
    "AstTypeTypeof",
    "AstTypeOptional",
    "AstTypeUnion",
    "AstTypeIntersection",
    "AstTypeSingletonBool",
    "AstTypeSingletonString",
    "AstTypeGroup",
    "AstTypeError",
    "AstTypePackExplicit",
    "AstTypePackVariadic",
    "AstTypePackGeneric",
  ];

  #[test]
  fn rtti_indices_unique() {
    let collisions = collect_collisions(RTTI_NAMES);
    assert!(
      collisions.is_empty(),
      "AST RTTI index collisions: {collisions:?}"
    );
  }

  #[test]
  fn rtti_index_is_stable_and_positive() {
    // Stability: the value is a pure function of the name.
    assert_eq!(
      ast_rtti_index("AstExprGroup"),
      ast_rtti_index("AstExprGroup")
    );
    // Positivity: leaves room for negative sentinels.
    assert!(ast_rtti_index("AstExprGroup") >= 0);
  }

  /// The full set of `LUAU_CST_RTTI(Class)` names in `Cst.h`. CST nodes share
  /// the index function with AST nodes but a separate index *space*, so only
  /// CST-vs-CST uniqueness matters.
  const CST_RTTI_NAMES: &[&str] = &[
    "CstExprGroup",
    "CstExprConstantNumber",
    "CstExprConstantInteger",
    "CstExprConstantString",
    "CstExprCall",
    "CstExprIndexExpr",
    "CstExprFunction",
    "CstExprTable",
    "CstExprOp",
    "CstExprTypeAssertion",
    "CstExprIfElse",
    "CstExprInterpString",
    "CstExprExplicitTypeInstantiation",
    "CstStatDo",
    "CstStatRepeat",
    "CstStatReturn",
    "CstStatLocal",
    "CstStatFor",
    "CstStatForIn",
    "CstStatAssign",
    "CstStatCompoundAssign",
    "CstStatFunction",
    "CstStatLocalFunction",
    "CstGenericType",
    "CstGenericTypePack",
    "CstStatTypeAlias",
    "CstStatTypeFunction",
    "CstTypeReference",
    "CstTypeTable",
    "CstTypeFunction",
    "CstTypeTypeof",
    "CstTypeUnion",
    "CstTypeIntersection",
    "CstTypeSingletonString",
    "CstTypeGroup",
    "CstTypePackExplicit",
    "CstTypePackGeneric",
  ];

  #[test]
  fn cst_rtti_indices_unique() {
    let collisions = collect_collisions(CST_RTTI_NAMES);
    assert!(
      collisions.is_empty(),
      "CST RTTI index collisions: {collisions:?}"
    );
  }
}
