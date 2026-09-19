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
  ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
  ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
  ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
  ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
  ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
  ast_expr_if_else::AstExprIfElse, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_index_name::AstExprIndexName, ast_expr_instantiate::AstExprInstantiate,
  ast_expr_interp_string::AstExprInterpString, ast_expr_local::AstExprLocal,
  ast_expr_table::AstExprTable, ast_expr_type_assertion::AstExprTypeAssertion,
  ast_expr_unary::AstExprUnary, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  ast_stat::AstStat, ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock,
  ast_stat_break::AstStatBreak, ast_stat_class::AstStatClass,
  ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_continue::AstStatContinue,
  ast_stat_declare_extern_type::AstStatDeclareExternType,
  ast_stat_declare_function::AstStatDeclareFunction, ast_stat_declare_global::AstStatDeclareGlobal,
  ast_stat_error::AstStatError, ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor,
  ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
  ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
  ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
  ast_stat_type_alias::AstStatTypeAlias, ast_stat_type_function::AstStatTypeFunction,
  ast_stat_while::AstStatWhile, ast_type::AstType, ast_type_error::AstTypeError,
  ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
  ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
  ast_type_pack::AstTypePack, ast_type_reference::AstTypeReference,
  ast_type_singleton_bool::AstTypeSingletonBool, ast_type_singleton_string::AstTypeSingletonString,
  ast_type_table::AstTypeTable, ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  cst_node::CstNode,
};

/// FNV-1a 32 位参数：offset basis 与 prime。
const FNV1A_OFFSET_BASIS: u32 = 0x811c_9dc5;
const FNV1A_PRIME: u32 = 0x0100_0193;
/// 折叠掩码：保持索引为正值，为负数哨兵（如 -1）留位。
const CLASS_INDEX_MASK: u32 = 0x7fff_ffff;

/// Stable per-type class index, the analog of `AstRtti<Class>::value`. FNV-1a
/// over the class name, folded to a positive `i32` so it can never collide with
/// a future "no class" sentinel (e.g. `-1`).
pub const fn ast_rtti_index(name: &str) -> i32 {
  // 切片模式逐字节消费：const fn 兼容且免索引越界检查
  let mut hash: u32 = FNV1A_OFFSET_BASIS;
  let mut bytes = name.as_bytes();
  while let [b, rest @ ..] = bytes {
    hash ^= *b as u32;
    hash = hash.wrapping_mul(FNV1A_PRIME);
    bytes = rest;
  }
  (hash & CLASS_INDEX_MASK) as i32
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

/// `ast_node_try_as::<T>(node)` 的可变形态：独占借用进、`Option<&mut T>` 出。
///
/// 输出生命周期由入参借用供给（不再凭空造 `'static`）：调用方交出的
/// `&mut AstNode` 本身就是该节点在借用期内独占证明，下转只是把同一个
/// place 的所有权形态换成派生类型（`#[repr(C)]` 单继承保证基址重合）。
/// 只读场景请用 [`ast_node_try_as`]。
///
/// # Safety
/// `node` 必须指向 arena 中存活的 repr(C) 节点（同 [`ast_node_as`]），且调用方在
/// 返回引用的整个生命周期内确实独占该节点（arena 无其他并发借用）。
#[inline]
pub unsafe fn ast_node_try_as_mut<T: AstNodeClass>(node: &mut AstNode) -> Option<&mut T> {
  if node.class_index == T::CLASS_INDEX {
    // SAFETY: 动态类型已由 class_index 判定，`&mut AstNode` 的独占借用经
    // repr(C) 首字段（基址重合）转写为 `&mut T`，与原借用同一 place、同一
    // 生命周期，不产生并发别名。
    Some(unsafe { &mut *(node as *mut AstNode).cast::<T>() })
  } else {
    None
  }
}

/// 基类家族判别：`node->asExpr()` 的判别面（cpp `AstNode::asExpr()` 只在 21 个
/// `AstExpr` 派生类上非空）。与 [`is_stat_class`] 同理，判别表集中于此，供
/// `as_expr`（可变）与 `as_expr_const`（只读）共用。
#[inline]
pub fn is_expr_class(class_index: i32) -> bool {
  matches!(
    class_index,
    AstExprBinary::CLASS_INDEX
      | AstExprCall::CLASS_INDEX
      | AstExprConstantBool::CLASS_INDEX
      | AstExprConstantInteger::CLASS_INDEX
      | AstExprConstantNil::CLASS_INDEX
      | AstExprConstantNumber::CLASS_INDEX
      | AstExprConstantString::CLASS_INDEX
      | AstExprError::CLASS_INDEX
      | AstExprFunction::CLASS_INDEX
      | AstExprGlobal::CLASS_INDEX
      | AstExprGroup::CLASS_INDEX
      | AstExprIfElse::CLASS_INDEX
      | AstExprIndexExpr::CLASS_INDEX
      | AstExprIndexName::CLASS_INDEX
      | AstExprInstantiate::CLASS_INDEX
      | AstExprInterpString::CLASS_INDEX
      | AstExprLocal::CLASS_INDEX
      | AstExprTable::CLASS_INDEX
      | AstExprTypeAssertion::CLASS_INDEX
      | AstExprUnary::CLASS_INDEX
      | AstExprVarargs::CLASS_INDEX
  )
}

/// 基类家族判别：`node->asStat()` 的判别面（cpp `AstNode::asStat()` 只在 21 个
/// `AstStat` 派生类上非空）。类索引判别与指针转换解耦，供 `as_stat`（可变）与
/// `as_stat_const`（只读）共用，避免两份 `matches!` 表漂移。
#[inline]
pub fn is_stat_class(class_index: i32) -> bool {
  matches!(
    class_index,
    AstStatAssign::CLASS_INDEX
      | AstStatBlock::CLASS_INDEX
      | AstStatBreak::CLASS_INDEX
      | AstStatClass::CLASS_INDEX
      | AstStatCompoundAssign::CLASS_INDEX
      | AstStatContinue::CLASS_INDEX
      | AstStatDeclareExternType::CLASS_INDEX
      | AstStatDeclareFunction::CLASS_INDEX
      | AstStatDeclareGlobal::CLASS_INDEX
      | AstStatError::CLASS_INDEX
      | AstStatExpr::CLASS_INDEX
      | AstStatFor::CLASS_INDEX
      | AstStatForIn::CLASS_INDEX
      | AstStatFunction::CLASS_INDEX
      | AstStatIf::CLASS_INDEX
      | AstStatLocal::CLASS_INDEX
      | AstStatLocalFunction::CLASS_INDEX
      | AstStatRepeat::CLASS_INDEX
      | AstStatReturn::CLASS_INDEX
      | AstStatTypeAlias::CLASS_INDEX
      | AstStatTypeFunction::CLASS_INDEX
      | AstStatWhile::CLASS_INDEX
  )
}

/// 基类家族判别：`node->asType()` 的判别面（cpp `AstNode::asType()` 只在 11 个
/// `AstType` 派生类上非空）。与 [`is_expr_class`] 同理，供 `as_type`（可变）与
/// `as_type_const`（只读）共用。
#[inline]
pub fn is_type_class(class_index: i32) -> bool {
  matches!(
    class_index,
    AstTypeError::CLASS_INDEX
      | AstTypeFunction::CLASS_INDEX
      | AstTypeGroup::CLASS_INDEX
      | AstTypeIntersection::CLASS_INDEX
      | AstTypeOptional::CLASS_INDEX
      | AstTypeReference::CLASS_INDEX
      | AstTypeSingletonBool::CLASS_INDEX
      | AstTypeSingletonString::CLASS_INDEX
      | AstTypeTable::CLASS_INDEX
      | AstTypeTypeof::CLASS_INDEX
      | AstTypeUnion::CLASS_INDEX
  )
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
