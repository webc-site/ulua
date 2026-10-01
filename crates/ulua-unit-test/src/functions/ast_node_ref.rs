//! 测试侧统一的"节点槽位 → 引用"入口（全安全门面，零 `unsafe`）。
//!
//! 测试代码里原本散落的三步式（`rtti::ast_node_try_as*` 下转 + 判空 +
//! `unsafe { &*p }`）在这里收口：null 分支进 `Option`，裸指针的**解引用**全部
//! 委托给 ulua-ast 已审计的句柄门面（`OptNode::from_ptr` + `Node::get` /
//! `AstArray::iter_nodes` / `Nodes::get`），本模块与调用点都不再出现 `unsafe`。
//! 语义与 cpp 的 `node->as<T>()`（失败返回 nullptr）、裸指针解引用、
//! `array.data[i]` 逐条对应。
//!
//! 调用约定：AST 字段仍是 `*mut AstExpr` 形态的，调用点先经
//! `OptNode::from_ptr(field)` 物化成句柄（一步、安全），再走本模块的
//! `as_ref_opt()` / `as_node::<T>()` 读取；句柄是 `Copy` 值，借用半径由句柄
//! 所在的局部绑定给出，比旧"槽位指针直读"更窄且可证明。已句柄化
//! （`Node`/`OptNode`/`Nodes`）或引用化的字段直接调用同名门面即可。
//!
//! 下转统一委托 `ulua_ast::rtti::ast_node_try_as` / `cst_node_try_as`，本模块
//! 不自行比对 `class_index`。指针身份（map key）保留 [`node_key`]：它只取址
//! 不解引用，是 `CstNodeMap` 等（以指针为 key 的）既有 map API 的桥接口。

use std::ptr::from_ref;

use ulua_ast::{
  records::{
    ast_array::AstArray,
    ast_expr::AstExpr,
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_type::AstType,
    ast_type_pack::AstTypePack,
    cst_node::CstNode,
    node_handle::{Node, Nodes, OptNode},
  },
  rtti::{AstNodeClass, AstNodeView, CstNodeClass, ast_node_try_as, cst_node_try_as},
};

/// 可空节点槽的非空读法：null 折叠为 `None`，返回引用借用句柄本体。
pub trait PtrRef<T> {
  /// cpp 直接解引用 `ptr`：null → `None`。
  fn as_ref_opt(&self) -> Option<&T>;
}

impl<T> PtrRef<T> for Node<T> {
  #[inline]
  fn as_ref_opt(&self) -> Option<&T> {
    Some(self.get())
  }
}

impl<T> PtrRef<T> for OptNode<T> {
  #[inline]
  fn as_ref_opt(&self) -> Option<&T> {
    self.get()
  }
}

/// 节点槽的下转读法：cpp `node->as<T>()`（下转与非空判定合为一次）。
pub trait NodePtr {
  /// 类型不符（或槽为空）→ `None`。返回引用的生命周期由 `&self` 给出。
  fn as_node<T: AstNodeClass>(&self) -> Option<&T>;
}

/// 句柄形态：`OptNode::from_ptr(field).as_node::<T>()` 一步完成判空 + 下转。
impl<T: AstNodeView> NodePtr for Node<T> {
  #[inline]
  fn as_node<U: AstNodeClass>(&self) -> Option<&U> {
    ast_node_try_as(self.get())
  }
}

impl<T: AstNodeView> NodePtr for OptNode<T> {
  #[inline]
  fn as_node<U: AstNodeClass>(&self) -> Option<&U> {
    self.get().and_then(ast_node_try_as)
  }
}

/// 引用形态：已物化为 `&AstExpr` 等基类引用的调用点直接链式下转，判型语义
/// 与句柄形态一致（repr(C) 单继承使基址重合，收口到安全版 `ast_node_try_as`）。
impl NodePtr for AstExpr {
  #[inline]
  fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
    ast_node_try_as(&self.base)
  }
}

impl NodePtr for AstStat {
  #[inline]
  fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
    ast_node_try_as(&self.base)
  }
}

impl NodePtr for AstType {
  #[inline]
  fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
    ast_node_try_as(&self.base)
  }
}

impl NodePtr for AstTypePack {
  #[inline]
  fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
    ast_node_try_as(&self.base)
  }
}

impl NodePtr for AstNode {
  #[inline]
  fn as_node<T: AstNodeClass>(&self) -> Option<&T> {
    ast_node_try_as(self)
  }
}

/// CST 节点槽的下转读法：`OptNode::from_ptr(cst_ptr).as_cst::<T>()`。
/// CST 是独立 RTTI 索引空间，下转走 ulua-ast 侧的 `cst_node_try_as`。
pub trait CstNodePtr {
  /// cpp `cstNode->as<T>()`：下转与非空判定合为一次，类型不符 → `None`。
  fn as_cst<T: CstNodeClass>(&self) -> Option<&T>;
}

impl CstNodePtr for OptNode<CstNode> {
  #[inline]
  fn as_cst<T: CstNodeClass>(&self) -> Option<&T> {
    self.get().and_then(cst_node_try_as)
  }
}

/// "节点槽数组"的下标读法：未迁移的 `AstArray<*mut U>` 与句柄化后的
/// `Nodes<U>` 在测试里同形（cpp `array.data[i]`），统一折叠为 `Option<&U>`。
///
/// `impl NodeSlots<U>`：实现集封闭，单态化消除 vtable 间接调用；调用点的
/// `::<T, _>` turbofish 对参数位 impl Trait 仍可用（`_` 占位）。
pub trait NodeSlots<U> {
  /// cpp `array.data[index]`（元素为节点指针/句柄）：null 或越界 → `None`，
  /// 否则给出该槽节点的共享引用（解引用收口在 ulua-ast 的
  /// `AstArray::iter_nodes` / `Nodes::get`）。
  fn slot<'a>(&'a self, index: usize) -> Option<&'a U>
  where
    U: 'a;
}

impl<U> NodeSlots<U> for AstArray<*mut U> {
  #[inline]
  fn slot<'a>(&'a self, index: usize) -> Option<&'a U>
  where
    U: 'a,
  {
    self.iter_nodes().nth(index)
  }
}

impl<U> NodeSlots<U> for Nodes<U> {
  #[inline]
  fn slot<'a>(&'a self, index: usize) -> Option<&'a U>
  where
    U: 'a,
  {
    self.get(index)
  }
}

/// cpp `array.data[i]->as<T>()`：元素为节点指针的数组取下标并下转。
/// 返回引用的生命周期由 `array` 的借用给出。
pub fn as_node_at<'a, T: AstNodeClass, U: AstNodeView + 'a>(
  array: &'a impl NodeSlots<U>,
  index: usize,
) -> Option<&'a T> {
  array.slot(index).and_then(ast_node_try_as)
}

/// cpp `array.data[i]`（元素为指针）：取下标并解引用，null → `None`。
/// 返回引用的生命周期由 `array` 的借用给出。
pub fn deref_at<'a, T: 'a>(array: &'a impl NodeSlots<T>, index: usize) -> Option<&'a T> {
  array.slot(index)
}

/// cpp `array.data[i]`：按下标取元素引用（元素是指针还是值都由调用点决定）。
/// 下标越界直接 panic，与测试断言语义一致。
pub fn elem<T>(array: &AstArray<T>, index: usize) -> &T {
  &array[index]
}

/// 以节点地址作 map key 时（如 `CstNodeMap`）由引用回推身份指针。
/// 只取址不解引用，是既有指针键 map API 的桥接口。
pub fn node_key<T>(node: &T) -> *mut AstNode {
  from_ref(node).cast::<AstNode>().cast_mut()
}
