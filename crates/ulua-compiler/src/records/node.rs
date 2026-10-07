//! AST 节点地址句柄（review §2「arena/自引用图 → 新type 句柄」的本 crate 收口）。
//!
//! cpp 侧 `DenseHashMap<AstX*, _>` / `Vec<AstLocal*>` 的裸指针在这里只承担
//! **节点身份**（哈希、相等、集合元素），与解引用无关：句柄封装 `NonNull`，
//! 天然非空、null 槽不再可能充当哨兵混入键空间。
//!
//! 读取节点只经 [`Node::get`] / [`Node::get_mut`] 两个**接收者绑定**的方法：
//! 返回借用的生命周期由句柄所在的 place（`&self` / `&mut self`）决定，而不是
//! 由调用方挑选的泛型 `'a` 拼装 —— 后者会让同一节点在类型系统之外获得任意
//! 长寿的 `&`/`&mut`，正是 review.md 要消灭的 noalias/别名 UB 形态。动态类型
//! 下转（cpp `node->as<T>()`）同样挂在句柄上（[`Node::try_as`] /
//! [`Node::try_as_mut`] / [`Node::is`]），故业务调用点既不出现 `*mut AstX`，
//! 也不出现 `unsafe`：全 crate 的 arena 解引用只落在本文件这两处。
//!
//! # Safety（`get` / `get_mut` 共同的 arena 契约，全部句柄来源共同满足）
//!
//! 1. 句柄由 [`Node::new`] / [`Node::from_ref`] / [`Node::from_mut`] /
//!    [`Node::from_ast_handle`] 建立，指向 ulua-parser arena 接线、长寿于本次
//!    编译的节点（arena 页在解析会话结束前不移动、不释放）；
//! 2. `get` 的共享借用区间内，visitor/compiler 对该节点只读；
//! 3. `get_mut` 的独占借用区间内，调用方是该子树唯一写者，且写穿只落在节点的
//!    编译期临时字段 —— 与 cpp 直接 `node->field` 解引用同一前提；`&mut self`
//!    只证明句柄 place 独占，目标节点的独占性仍由本契约承担（与
//!    `ulua_ast::records::node_handle::Node::get_mut` 同一纪律）。

use core::{
  fmt,
  hash::{Hash, Hasher as StdHasher},
  ptr::{NonNull, from_mut, from_ref},
};

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, node_handle},
  rtti::{
    AstNodeClass, AstNodeView, AstNodeViewMut, ast_node_is, ast_node_try_as, ast_node_try_as_mut,
  },
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

  /// 可空 cpp 形槽位（`*mut T` 承载「缺席」哨兵）→ 句柄：null 归一为 `None`，
  /// 与 [`Node::new`] 的「null 折 dangling」互补——前者用于键（哨兵可存取），
  /// 本用于值/可选节点（哨兵即缺席，§2 可空指针 → `Option`）。
  #[inline]
  pub(crate) fn try_new(p: *mut T) -> Option<Self> {
    NonNull::new(p).map(|ptr| Self { ptr })
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

  /// ulua-ast 句柄（`Node::from_raw` 已证非空）升格为本 crate 地址句柄：
  /// 表索引器子图句柄化后，AST 字段即句柄形态，身份同源（同一 arena 地址）。
  #[inline]
  pub(crate) fn from_ast_handle(node: node_handle::Node<T>) -> Self {
    Self::new(node.as_ptr())
  }

  /// 交回等价的裸节点地址，供 ulua-ast 的指针形参 API（visit/deref 门面）使用。
  #[inline]
  pub(crate) fn as_ptr(self) -> *mut T {
    self.ptr.as_ptr()
  }

  /// 只读借用节点。借用半径即本句柄的借用（`&self`），不得跨出句柄所在 place
  /// 留存——句柄由调用方以本地绑定持有，从而「节点存活」由 arena 契约 +
  /// 句柄存活共同兑现，不再有调用方自选的 `'a`。
  ///
  /// # Safety
  /// 模块头 arena 契约 1、2：句柄指向长寿节点，借用区间内无写者并发。
  /// `NonNull::new` 已在全部构造点排除 null。
  #[inline]
  pub(crate) fn get(&self) -> &T {
    // Safety: 见方法级 `# Safety`。
    unsafe { self.ptr.as_ref() }
  }

  /// 可变借用节点。借用半径即本 `&mut` 句柄的借用，对同一节点的并发可变借用由
  /// 调用方独占性保证（cpp 同前提）。
  ///
  /// # Safety
  /// 模块头 arena 契约 1、3：节点长寿，且本次借用区间内调用方是该子树唯一写者。
  #[inline]
  pub(crate) fn get_mut(&mut self) -> &mut T {
    // Safety: 见方法级 `# Safety`；`&mut self` 只保证句柄 place 独占，目标节点
    // 的独占由上述契约承担（同 ulua-ast `node_handle::Node::get_mut`）。
    unsafe { self.ptr.as_mut() }
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

/// 动态类型判型/下转（cpp `node->as<T>()` / `node->is<T>()`）：入参借用即存活
/// 证明，输出借用与入参同半径，全程 safe（解引用只发生在 [`Node::get`]）。
impl<T: AstNodeView> Node<T> {
  /// class_index 命中才借出派生视图；不匹配返回 `None`。
  #[inline]
  pub(crate) fn try_as<U: AstNodeClass>(&self) -> Option<&U> {
    ast_node_try_as::<U>(self.get().as_ast_node())
  }

  /// 纯判型形态（不外传借用）。
  #[inline]
  pub(crate) fn is<U: AstNodeClass>(&self) -> bool {
    ast_node_is::<U>(self.get().as_ast_node())
  }
}

/// [`Node::try_as`] 的独占形态：解引用经 [`Node::get_mut`]，
/// 独占半径由 `&mut self` 句柄借用供给。
impl<T: AstNodeViewMut> Node<T> {
  /// class_index 命中才交出派生类型的独占借用。
  #[inline]
  pub(crate) fn try_as_mut<U: AstNodeClass>(&mut self) -> Option<&mut U> {
    ast_node_try_as_mut::<U>(self.get_mut().as_ast_node_mut())
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
  /// 将表达式句柄下转为具体引用枚举（先借出只读视图，半径同 [`Node::get`]）。
  #[inline]
  pub(crate) fn as_expr_ref(&self) -> AstExprRef<'_> {
    self.get().as_expr_ref()
  }
}
