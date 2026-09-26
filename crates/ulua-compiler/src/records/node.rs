//! AST 节点地址句柄（review §2「非空指针 → 索引句柄」的本 crate 收口）。
//!
//! cpp 侧 `DenseHashMap<AstX*, _>` / `Vec<AstLocal*>` 的裸指针在这里只承担
//! **节点身份**（哈希、相等、集合元素），与解引用无关：句柄封装 `NonNull`，
//! 天然非空、null 槽不再可能充当哨兵混入键空间；读取节点需经显式
//! [`Node::borrow`] / [`Node::borrow_mut`]，把「节点存活」这一 arena 契约
//! 收口到本文件两行 `unsafe`，业务代码不再出现 `*mut AstX` 与散点解引用。

use core::{
  fmt,
  hash::{Hash, Hasher as StdHasher},
  ptr::{NonNull, from_mut, from_ref},
};

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, node_handle},
};
use ulua_common::records::dense_hash_table::DenseDefault;

/// 指向 arena 节点的地址句柄。
///
/// 等值/哈希按**节点地址**（同一 arena 分配地址即同一身份），与 cpp 指针键
/// 逐位一致；`Copy`，尺寸与裸指针相同（`Option<Node<T>>` 复用 null niche）。
pub struct Node<T> {
  ptr: NonNull<T>,
}

// 手写实现（derive 会给 `T` 加 `Clone/Debug/PartialEq` 约束，而节点类型没有、
// 也不需要）：一切按地址身份，与 `T` 自身的 trait 实现无关。
impl<T> Clone for Node<T> {
  #[inline]
  fn clone(&self) -> Self {
    *self
  }
}
impl<T> Copy for Node<T> {}
impl<T> PartialEq for Node<T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.ptr.as_ptr() == other.ptr.as_ptr()
  }
}
impl<T> Eq for Node<T> {}
impl<T> fmt::Debug for Node<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Node({:p})", self.ptr.as_ptr())
  }
}

impl<T> Node<T> {
  /// 包装一个 arena 节点地址。按全 crate 契约「AST 节点地址恒非空、null 从不
  /// 作为有效键/句柄值」；防御起见 null 折叠为唯一悬垂占位（等价旧指针键表
  /// 对 null 键的哨兵可存取语义，且永不与真实 arena 地址相撞）。
  #[inline]
  pub(crate) fn new(p: *mut T) -> Self {
    Self {
      ptr: NonNull::new(p).unwrap_or_else(NonNull::dangling),
    }
  }

  /// 共享视图升句柄（键常由 `&`/`&mut` 借用现造）。
  #[inline]
  pub(crate) fn from_ref(r: &T) -> Self {
    Self::new(from_ref(r).cast_mut())
  }

  /// `*const` 侧句柄（如类型图中的 `&AstType` 视图）：只做只读用途。
  #[inline]
  pub(crate) fn from_const(p: *const T) -> Self {
    Self::new(p.cast_mut())
  }

  /// 交回等价的裸节点地址，供 ulua-ast 的指针形参 API（visit/deref 门面）使用。
  #[inline]
  pub(crate) fn as_ptr(self) -> *mut T {
    self.ptr.as_ptr()
  }

  /// 只读借用节点。生命周期半径由调用点推断，不得跨出本次编译留存。
  ///
  /// 契约（全部句柄来源共同满足，见 [`Node::new`]）：句柄指向 parser arena
  /// 接线、长寿于本次编译的节点；读区间内 visitor/compiler 对该节点只读或
  /// 独占写，与 cpp 直接 `node->field` 解引用同一前提。
  // Safety: `NonNull::new` 已在构造点排除 null；存活/对齐/只读并发由上述契约承担。
  #[inline]
  pub(crate) fn borrow<'a>(self) -> &'a T {
    unsafe { self.ptr.as_ref() }
  }

  /// 可变借用节点。对同一节点的并发可变借用由调用方独占性保证（cpp 同前提）。
  // Safety: 同 `borrow`，另要求调用方此刻持有该子树唯一写权限。
  #[inline]
  pub(crate) fn borrow_mut<'a>(self) -> &'a mut T {
    // `NonNull::as_mut` 要求 `&mut self`，而句柄是 `Copy` 的地址值：
    // 直接按指针重建可变引用，语义等同。
    unsafe { &mut *self.ptr.as_ptr() }
  }

  /// 基/派生（repr(C) 前缀重合）或同类节点间的地址重转，等价裸指针 `cast`。
  #[inline]
  pub(crate) fn cast<U>(self) -> Node<U> {
    Node::new(self.ptr.cast::<U>().as_ptr())
  }

  /// `&mut T` → 句柄的显式写法（与 ulua-ast `from_mut` 惯用法对齐）。
  #[inline]
  pub(crate) fn from_mut(r: &mut T) -> Self {
    Self::new(from_mut(r))
  }
}

impl<T> From<*mut T> for Node<T> {
  #[inline]
  fn from(p: *mut T) -> Self {
    Self::new(p)
  }
}

/// ulua-ast 句柄化字段（`records::node_handle::Node`）→ 本 crate 身份句柄：
/// 同一 arena 地址的直接交接，无借用产生。
impl<T> From<node_handle::Node<T>> for Node<T> {
  #[inline]
  fn from(n: node_handle::Node<T>) -> Self {
    Self::new(n.as_ptr())
  }
}

impl<T> From<&mut T> for Node<T> {
  #[inline]
  fn from(r: &mut T) -> Self {
    Self::from_mut(r)
  }
}

impl<T> From<&T> for Node<T> {
  #[inline]
  fn from(r: &T) -> Self {
    Self::from_ref(r)
  }
}

impl<T> From<*const T> for Node<T> {
  #[inline]
  fn from(p: *const T) -> Self {
    Self::from_const(p)
  }
}

impl<T> Hash for Node<T> {
  #[inline]
  fn hash<H: StdHasher>(&self, state: &mut H) {
    self.ptr.addr().hash(state);
  }
}

impl<T> Default for Node<T> {
  #[inline]
  fn default() -> Self {
    Self {
      ptr: NonNull::dangling(),
    }
  }
}

/// 指针键表 `Default` 占位槽：与旧 `*mut` 键的 null 占位等价——只参与存储，
/// 占用判定由位图负责，悬垂地址永不与真实 arena 节点相撞（见 [`Node::new`]）。
impl<T> DenseDefault for Node<T> {
  #[inline]
  fn dense_default() -> Self {
    Self {
      ptr: NonNull::dangling(),
    }
  }
}

impl Node<AstExpr> {
  /// 将表达式句柄下转为具体引用枚举（先借出只读视图）。
  #[inline]
  pub(crate) fn as_expr_ref<'a>(self) -> AstExprRef<'a> {
    self.borrow().as_expr_ref()
  }
}
