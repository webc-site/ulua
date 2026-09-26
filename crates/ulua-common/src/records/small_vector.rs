//! `Luau::SmallVector<T, N>` 的 Rust 对应物，委托 `smallvec`。
//!
//! 参考：`luau/Common/include/Luau/SmallVector.h`（整个文件），Luau 上游。
//! 小缓冲优化（SBO）向量：前 `N` 个元素内联存放，增长时落到堆块。原手写
//! alloc/unsafe 核心已换成 `smallvec` crate；C++ 形状的方法名（`size`/
//! `push_back`/`emplace_back`/...）保留在 newtype 上，下游 crate 无需改动。
//! 每个方法都是直接的内联透传 —— 相比手写版本无额外间接层。
//!
//! 刻意偏差（不可观察且不更慢）：
//! - 增长策略用 smallvec 的均摊倍增，替代 C++ 的 `1.5x`-或-`+4`
//!   （realloc 次数不增，内联容量-`N` 语义不变），
//! - `heap` 指针的 move-safety 舞步完全消失：`SmallVec` 拥有表示，
//!   本文件不含任何 `unsafe`。

use alloc::vec::Vec;
use core::{
  borrow::{Borrow, BorrowMut},
  fmt,
  hash::{Hash, Hasher},
  ops::{Deref, DerefMut},
  slice,
};

use smallvec::SmallVec;

use crate::records::dense_hash_table::DenseDefault;

pub struct SmallVector<T, const N: usize>(SmallVec<[T; N]>);

impl<T, const N: usize> SmallVector<T, N> {
  #[inline]
  pub fn new() -> Self {
    SmallVector(SmallVec::new())
  }

  #[inline]
  pub fn as_slice(&self) -> &[T] {
    &self.0
  }

  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    &mut self.0
  }

  #[inline]
  pub fn size(&self) -> u32 {
    self.0.len() as u32
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  #[inline]
  pub fn capacity(&self) -> u32 {
    self.0.capacity() as u32
  }

  #[inline]
  pub fn empty(&self) -> bool {
    self.0.is_empty()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  #[inline]
  pub fn front(&self) -> &T {
    self.0.first().expect("front on empty SmallVector")
  }

  #[inline]
  pub fn back(&self) -> &T {
    self.0.last().expect("back on empty SmallVector")
  }

  /// Vec 风格名（bytecode/code-gen 的图构建在用）；与 `push_back` 同实现。
  #[inline]
  pub fn push(&mut self, value: T) {
    self.0.push(value);
  }

  #[inline]
  pub fn push_back(&mut self, value: T) {
    self.0.push(value);
  }

  /// `emplace_back` collapses to `push_back` of the constructed value; Rust has
  /// no in-place variadic construction, and the move is free.
  #[inline]
  pub fn emplace_back(&mut self, value: T) -> &mut T {
    self.0.push(value);
    self.0.last_mut().expect("push_back 保证非空")
  }

  #[inline]
  pub fn pop_back(&mut self) {
    assert!(!self.0.is_empty());
    self.0.pop();
  }

  #[inline]
  pub fn clear(&mut self) {
    self.0.clear();
  }

  /// 绝对容量语义（对齐 C++ `reserve`）：保证 `capacity() >= reserve_size`，
  /// 而非 `Vec::reserve` 的 `len + additional`。
  #[inline]
  pub fn reserve(&mut self, reserve_size: u32) {
    let reserve_size = reserve_size as usize;
    if reserve_size > self.0.capacity() {
      // 进入该分支时 `reserve_size > capacity >= len`，相减无下溢。
      self.0.reserve(reserve_size - self.0.len());
    }
  }
}

impl<T: Default, const N: usize> SmallVector<T, N> {
  #[inline]
  pub fn resize(&mut self, new_size: u32) {
    // `resize_with` 而非 `resize`：保持原 API 只约束 `T: Default`
    // （smallvec 的 `resize` 额外要求 `Clone`）。
    self.0.resize_with(new_size as usize, T::default);
  }
}

impl<T, const N: usize> Default for SmallVector<T, N> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<T, const N: usize> Deref for SmallVector<T, N> {
  type Target = [T];
  #[inline]
  fn deref(&self) -> &[T] {
    self.as_slice()
  }
}

impl<T, const N: usize> DerefMut for SmallVector<T, N> {
  #[inline]
  fn deref_mut(&mut self) -> &mut [T] {
    self.as_mut_slice()
  }
}

impl<T: Clone, const N: usize> Clone for SmallVector<T, N> {
  #[inline]
  fn clone(&self) -> Self {
    SmallVector(self.0.clone())
  }
}

impl<T: PartialEq, const N: usize> PartialEq for SmallVector<T, N> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

impl<T: Eq, const N: usize> Eq for SmallVector<T, N> {}

impl<T: Hash, const N: usize> Hash for SmallVector<T, N> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_slice().hash(state);
  }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for SmallVector<T, N> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&**self, f)
  }
}

impl<'a, T, const N: usize> IntoIterator for &'a SmallVector<T, N> {
  type Item = &'a T;
  type IntoIter = slice::Iter<'a, T>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.as_slice().iter()
  }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SmallVector<T, N> {
  type Item = &'a mut T;
  type IntoIter = slice::IterMut<'a, T>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.as_mut_slice().iter_mut()
  }
}

impl<T, const N: usize> IntoIterator for SmallVector<T, N> {
  type Item = T;
  type IntoIter = smallvec::IntoIter<[T; N]>;
  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<T, const N: usize> FromIterator<T> for SmallVector<T, N> {
  #[inline]
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    SmallVector(iter.into_iter().collect())
  }
}

impl<T, const N: usize> From<SmallVec<[T; N]>> for SmallVector<T, N> {
  #[inline]
  fn from(vec: SmallVec<[T; N]>) -> Self {
    SmallVector(vec)
  }
}

impl<T, const N: usize> From<SmallVector<T, N>> for SmallVec<[T; N]> {
  #[inline]
  fn from(vec: SmallVector<T, N>) -> Self {
    vec.0
  }
}

impl<T, const N: usize> From<Vec<T>> for SmallVector<T, N> {
  #[inline]
  fn from(vec: Vec<T>) -> Self {
    SmallVector(SmallVec::from_vec(vec))
  }
}

impl<T, const N: usize> From<[T; N]> for SmallVector<T, N> {
  #[inline]
  fn from(arr: [T; N]) -> Self {
    SmallVector(SmallVec::from_buf(arr))
  }
}

impl<T, const N: usize> AsRef<[T]> for SmallVector<T, N> {
  #[inline]
  fn as_ref(&self) -> &[T] {
    self.as_slice()
  }
}

impl<T, const N: usize> AsMut<[T]> for SmallVector<T, N> {
  #[inline]
  fn as_mut(&mut self) -> &mut [T] {
    self.as_mut_slice()
  }
}

impl<T, const N: usize> Borrow<[T]> for SmallVector<T, N> {
  #[inline]
  fn borrow(&self) -> &[T] {
    self.as_slice()
  }
}

impl<T, const N: usize> BorrowMut<[T]> for SmallVector<T, N> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [T] {
    self.as_mut_slice()
  }
}

/// 标准集合 trait：逐元素追加，直接委托 `SmallVec` 的 `Extend`（内联路径
/// 不变，容量增长策略与 `push_back` 一致）。
impl<T, const N: usize> Extend<T> for SmallVector<T, N> {
  #[inline]
  fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
    self.0.extend(iter);
  }
}

impl<T, const N: usize> DenseDefault for SmallVector<T, N> {
  #[inline]
  fn dense_default() -> Self {
    Self::new()
  }
}
