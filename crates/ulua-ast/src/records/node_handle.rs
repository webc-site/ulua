//! arena 节点槽位的类型化句柄(`review.md` §2「arena/自引用图:把 unsafe 关进
//! 最小契约边界」的第一波落地,见 `review.md` §11 方向)。
//!
//! cpp 移植层的节点字段是「子节点裸指针 + AstArray 指针数组」,导致全部下游
//! (compiler/analysis)必须自带 `unsafe` 解引用与判空。本模块把存储面收进三个
//! 一次性封装:
//!
//! - [`Node<T>`]:非空子节点句柄(`NonNull` 承载,布局与裸指针逐位相同);
//! - [`OptNode<T>`]:可空子节点句柄,`None` 即 cpp 的 `nullptr`,哨兵消失;
//! - [`Nodes<T>`]:子节点句柄数组(`Box<[Node<T>]>`),替代「指针数组 + 长度」。
//!
//! 其读接口(`get`/`iter`)是全 crate 唯一的 arena 解引用点:节点由 `Allocator`
//! 的 bump 页承载、页在解析会话结束前从不移动/释放(parser 错误路径经
//! `Allocator::free_pages` 归还前树已放弃),且遍历写穿遵循「visitor 持有树根
//! `&mut` 期间单线程独占」的既有纪律(见 `visit.rs` 总说明)——该契约即以下全部
//! `// Safety:` 块的公共前提。字段引用化是渐进式:`AstStatBlock`/`AstExprFunction`
//! 主干已迁,其余 records 按消费方矩阵在后续波次跟进。

use alloc::{boxed::Box, vec::Vec};
use core::{
  fmt::{Debug, Formatter, Result},
  hash::{Hash, Hasher},
  ops::{Deref, DerefMut, Index},
  ptr::{NonNull, null_mut},
  slice::{Iter, IterMut},
};

use crate::records::temp_vector::TempVector;

/// 非空 arena 子节点句柄。`#[repr(transparent)]` 保证与 `*mut T` 逐位相同,
/// `#[repr(C)]` 节点布局(含 RTTI 首字段链)不受影响。
#[repr(transparent)]
pub struct Node<T> {
  ptr: NonNull<T>,
}

impl<T> Node<T> {
  /// 从 arena 分配结果建槽:cpp 中 `new` 失败即中止,分配器返回的槽恒非空,
  /// 违例(逻辑 bug)以 panic 拦截,不再让 null 渗透进类型。
  #[inline]
  pub(crate) fn from_raw(ptr: *mut T) -> Self {
    Self {
      ptr: NonNull::new(ptr).expect("arena 分配槽位恒非空(分配失败即中止)"),
    }
  }

  /// 已证非空的地址建槽(下游合成节点用:引用/NonNull 即非空性证明,safe)。
  #[inline]
  pub fn from_non_null(ptr: NonNull<T>) -> Self {
    Self { ptr }
  }

  /// 由存活引用建槽(safe:引用即非空 + 存活证明)。
  #[inline]
  pub fn from_ref(r: &T) -> Self {
    Self {
      ptr: NonNull::from(r),
    }
  }

  /// [`Node::from_ref`] 的独占形态。
  #[inline]
  pub fn from_mut(r: &mut T) -> Self {
    Self {
      ptr: NonNull::from(r),
    }
  }

  /// 只读视图:借用半径由 `&self` 供给,调用方拿不到裸指针。
  #[inline]
  pub fn get(&self) -> &T {
    // Safety: 模块级 arena 契约——NonNull 出自存活分配,页在树存活期内不移动不释放。
    unsafe { self.ptr.as_ref() }
  }

  /// 写穿视图(cpp `visit(AstVisitor*)` 非 const 语义的 Rust 形态):独占性由
  /// `&mut self` 继承——持有父节点 `&mut` 即持有本槽位的独占证明。
  #[inline]
  pub fn get_mut(&mut self) -> &mut T {
    // Safety: `&mut self` 保证本句柄所在 place 独占;目标节点同受 arena 存活契约保护。
    unsafe { self.ptr.as_mut() }
  }

  /// 既有裸指针门面(`rtti` 判型、`CstNodeMap` 键、FFI 边界)的桥接口;
  /// 新代码优先 [`Node::get`] / [`Node::get_mut`]。
  #[inline]
  pub fn as_ptr(&self) -> *mut T {
    self.ptr.as_ptr()
  }

  /// 指针身份比较(cpp `a == b` 的节点同一性,非内容比较)。
  #[inline]
  pub fn is_same(&self, other: &Node<T>) -> bool {
    self.ptr == other.ptr
  }

  /// 转换节点类型（布局保证与 NonNull::cast 一致）。
  #[inline]
  pub fn cast<U>(self) -> Node<U> {
    Node {
      ptr: self.ptr.cast(),
    }
  }
}

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
    self.is_same(other)
  }
}
impl<T> Eq for Node<T> {}

impl<T> Hash for Node<T> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.ptr.as_ptr().hash(state);
  }
}

impl<T: Debug> Debug for Node<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(self.get(), f)
  }
}

impl<T> Deref for Node<T> {
  type Target = T;
  /// 只读解引用糖:等价 [`Node::get`],让既有 `node.field` 读取形态原样保留。
  #[inline]
  fn deref(&self) -> &T {
    self.get()
  }
}

impl<T> DerefMut for Node<T> {
  /// 独占解引用糖:等价 [`Node::get_mut`]。
  #[inline]
  fn deref_mut(&mut self) -> &mut T {
    self.get_mut()
  }
}

/// 可空 arena 子节点句柄:cpp `T* = nullptr` 槽位的直接对应,null 哨兵在类型层消失。
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct OptNode<T> {
  ptr: Option<NonNull<T>>,
}

// 手写 Default/Copy/Clone:derive 会按字段类型误加 `T: Default` / `T: Copy` 约束,
// 而 `Option<NonNull<T>>` 的空槽与复制都与 T 的这些能力无关。
impl<T> Default for OptNode<T> {
  #[inline]
  fn default() -> Self {
    Self { ptr: None }
  }
}

impl<T> Clone for OptNode<T> {
  #[inline]
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Copy for OptNode<T> {}

impl<T> OptNode<T> {
  /// 由 `Option<NonNull<T>>` 建句柄:cpp 可空槽在构造端的非空性证明形态。
  #[inline]
  pub fn from_non_null(ptr: Option<NonNull<T>>) -> Self {
    Self { ptr }
  }

  /// 由原始裸指针构造可空句柄（null 映射为 None）。
  #[inline]
  pub fn from_ptr(ptr: *mut T) -> Self {
    Self {
      ptr: NonNull::new(ptr),
    }
  }

  #[inline]
  pub fn get(&self) -> Option<&T> {
    // Safety: 同 [`Node::get`] 的 arena 存活契约;None 即 cpp 的 nullptr,无解引用。
    self.ptr.map(|p| unsafe { p.as_ref() })
  }

  /// 槽位是否为空(cpp `p == nullptr` 判定的类型化形态)。
  #[inline]
  pub fn is_none(&self) -> bool {
    self.ptr.is_none()
  }

  /// 槽位是否已接线(cpp `p != nullptr` 判定的类型化形态)。
  #[inline]
  pub fn is_some(&self) -> bool {
    self.ptr.is_some()
  }

  #[inline]
  pub fn get_mut(&mut self) -> Option<&mut T> {
    // Safety: 同 [`Node::get_mut`],独占性由 `&mut self` 继承。
    self.ptr.map(|mut p| unsafe { p.as_mut() })
  }

  /// 桥接口,语义同 [`Node::as_ptr`]:null 落回 `nullptr` 供既有指针门面消费。
  #[inline]
  pub fn as_ptr(&self) -> *mut T {
    self.ptr.map_or(null_mut(), NonNull::as_ptr)
  }

  /// 判定是否为空句柄（等价 is_none）。
  #[inline]
  pub fn is_null(&self) -> bool {
    self.is_none()
  }

  /// 共享引用视图（兼容 Option::as_ref 习惯）。
  #[inline]
  pub fn as_ref(&self) -> Option<&T> {
    self.get()
  }

  /// 转为 `Option<Node<T>>`。
  #[inline]
  pub fn to_option(&self) -> Option<Node<T>> {
    self.ptr.map(Node::from_non_null)
  }

  /// [`Option::map`] 形态的取值:把 arena 存活的 `&T` 交给 `f`,只回传映射结果
  /// (cpp `p ? f(p) : 无` 的类型化收口)。挂在 `&self` 而非 `self`(Copy)上,
  /// 让 `U` 可以借用节点字段;`f` 拿不到裸指针。
  #[inline]
  pub fn map<'s, U>(&'s self, f: impl FnOnce(&'s T) -> U) -> Option<U> {
    self.get().map(f)
  }

  /// 空态回退到调用方给出的存活替身引用(cpp `p ? *p : fallback` 的读面)。
  /// 返回类型是共享引用而非 [`Node`]:替身未必出自 arena,强迫其包装成
  /// 句柄会把裸指针语义倒灌回调用端。
  #[inline]
  pub fn unwrap_or<'a>(&'a self, fallback: &'a T) -> &'a T {
    self.get().unwrap_or(fallback)
  }

  /// 转换节点类型（布局保证与 NonNull::cast 一致）。
  #[inline]
  pub fn cast<U>(self) -> OptNode<U> {
    OptNode {
      ptr: self.ptr.map(NonNull::cast),
    }
  }
}

impl<T: Debug> Debug for OptNode<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self.get() {
      Some(node) => Debug::fmt(node, f),
      None => f.write_str("None"),
    }
  }
}

/// 子节点句柄数组:`Vec<T>` 式所有权字段,替代 cpp「arena 指针数组 + 长度」。
/// 数组本体堆分配(不再占 arena),元素仍是指向 arena 节点的 [`Node`] 句柄。
pub struct Nodes<T> {
  items: Box<[Node<T>]>,
}

impl<T> Nodes<T> {
  #[inline]
  pub fn empty() -> Self {
    Self {
      items: Box::default(),
    }
  }

  #[inline]
  pub fn from_vec(items: Vec<Node<T>>) -> Self {
    Self {
      items: items.into_boxed_slice(),
    }
  }

  /// 从 arena 槽数组(裸指针切片)构造:parser 的 scratch/TempVector 收口点。
  /// 元素恒出自 arena 分配(cpp 语义下 nullptr 数组元素只在错误形态出现),
  /// 经 [`Node::from_raw`] 的单槽判空拦截。
  #[inline]
  pub(crate) fn from_raw_slice(sl: &[*mut T]) -> Self {
    Self::from_vec(sl.iter().copied().map(Node::from_raw).collect())
  }

  /// cpp `Parser::copy(const TempVector<T*>&)` 的句柄化形态:把 parser scratch
  /// 窗口 `[offset, offset + size_)` 里的 arena 节点槽**复制**为堆持有句柄数组
  /// (TempVector 在 Drop 时截回 scratch,本构造在截断前取走元素,所有权即完成
  /// 转移;空 scratch 窗口落 [`Nodes::empty`],对应 cpp `{null, 0}`)。
  #[inline]
  pub fn from_temp_vector(data: &TempVector<'_, *mut T>) -> Self {
    Self::from_raw_slice(data.as_slice())
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.items.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }

  /// 只读遍历(= 旧 `AstArray::iter_nodes`):元素直接是节点共享引用。
  #[inline]
  pub fn iter(&self) -> NodesIter<'_, T> {
    NodesIter {
      inner: self.items.iter(),
    }
  }

  /// 句柄遍历(= 旧 `AstArray::iter` 给出 `&*mut T` 的位置):需要
  /// `get_mut`/`as_ptr`/判型下转的写穿或身份消费路径使用。
  #[inline]
  pub fn iter_nodes(&self) -> impl DoubleEndedIterator<Item = &Node<T>> + ExactSizeIterator {
    self.items.iter()
  }

  /// 可变元素遍历:直接交出 arena 节点的可变引用(写穿子树循环的 Rust 形态)。
  ///
  /// 布局与契约均允许:`Box<[Node<T>]>` 的槽位互异,`&mut self` 给出槽位独占,
  /// 解引用走 [`Node::get_mut`] 同一 arena 契约;cpp parser 的数组元素各为独
  /// 立子节点(无重名槽),元素间 `&mut T` 不互相别名——与既有
  /// `iter_nodes_mut` + `DerefMut` 组合的别名前提完全一致,非本接口新增。
  #[inline]
  pub fn iter_mut(&mut self) -> NodesIterMut<'_, T> {
    NodesIterMut {
      inner: self.items.iter_mut(),
    }
  }

  /// 句柄切片的可变视图(对应 [`Nodes::as_slice`])。
  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [Node<T>] {
    &mut self.items
  }

  /// 独占句柄遍历:visitor 写穿子树(`AstVisitable::visit_children`)的入口,
  /// 可变性沿 `&mut self` 传递,全链路无裸指针。
  #[inline]
  pub fn iter_nodes_mut(
    &mut self,
  ) -> impl DoubleEndedIterator<Item = &mut Node<T>> + ExactSizeIterator {
    self.items.iter_mut()
  }

  #[inline]
  pub fn get(&self, index: usize) -> Option<&T> {
    self.items.get(index).map(Node::get)
  }

  /// 越界即 panic 的句柄下标(与旧 `as_slice()[i]` 行为一致)。
  #[inline]
  pub fn at(&self, index: usize) -> &Node<T> {
    &self.items[index]
  }

  #[inline]
  pub fn as_slice(&self) -> &[Node<T>] {
    &self.items
  }

  /// 长度（兼容旧 AstArray::size 习惯）。
  #[inline]
  pub fn size(&self) -> usize {
    self.len()
  }
}

impl<T> Index<usize> for Nodes<T> {
  type Output = Node<T>;
  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    &self.items[index]
  }
}

impl<T> From<Node<T>> for *mut T {
  #[inline]
  fn from(node: Node<T>) -> Self {
    node.as_ptr()
  }
}

impl<T> From<OptNode<T>> for *mut T {
  #[inline]
  fn from(node: OptNode<T>) -> Self {
    node.as_ptr()
  }
}

impl<T> AsRef<T> for Node<T> {
  #[inline]
  fn as_ref(&self) -> &T {
    self.get()
  }
}

impl<T> Default for Nodes<T> {
  #[inline]
  fn default() -> Self {
    Self::empty()
  }
}

impl<T> Clone for Nodes<T> {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      items: self.items.clone(),
    }
  }
}

impl<T> PartialEq for Nodes<T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.items == other.items
  }
}
impl<T> Eq for Nodes<T> {}

impl<T: Debug> Debug for Nodes<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Debug::fmt(&self.items, f)
  }
}

/// [`Nodes::iter`] 的具体迭代器(免 `dyn`,保单态化)。
pub struct NodesIter<'a, T> {
  inner: Iter<'a, Node<T>>,
}

impl<'a, T> Iterator for NodesIter<'a, T> {
  type Item = &'a T;
  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(Node::get)
  }
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl<T> ExactSizeIterator for NodesIter<'_, T> {}

/// [`Nodes::iter_mut`] 的具体迭代器(与 [`NodesIter`] 对称)。
pub struct NodesIterMut<'a, T> {
  inner: IterMut<'a, Node<T>>,
}

impl<'a, T> Iterator for NodesIterMut<'a, T> {
  type Item = &'a mut T;
  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(Node::get_mut)
  }
  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl<T> DoubleEndedIterator for NodesIterMut<'_, T> {
  #[inline]
  fn next_back(&mut self) -> Option<Self::Item> {
    self.inner.next_back().map(Node::get_mut)
  }
}

impl<T> ExactSizeIterator for NodesIterMut<'_, T> {}

impl<'a, T> IntoIterator for &'a Nodes<T> {
  type Item = &'a T;
  type IntoIter = NodesIter<'a, T>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Nodes<T> {
  type Item = &'a mut T;
  type IntoIter = NodesIterMut<'a, T>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}
