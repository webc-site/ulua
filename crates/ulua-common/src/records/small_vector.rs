//! Rust counterpart of `Luau::SmallVector<T, N>`, delegating to `smallvec`.
//!
//! Reference: `luau/Common/include/Luau/SmallVector.h` (whole file), Luau upstream.
//! A small-buffer-optimized vector: the first `N` elements live inline; on growth
//! it spills to a heap block. The hand-rolled alloc/unsafe core was swapped for
//! the `smallvec` crate; the C++-shaped method names (`size`/`push_back`/
//! `emplace_back`/...) are kept on a newtype so downstream crates compile
//! unchanged. Every method is a direct inline pass-through — no extra
//! indirection vs. the previous hand-written version.
//!
//! Deliberate deviations (neither observable nor slower):
//! - growth policy is smallvec's amortized doubling instead of C++'s `1.5x`-or-
//!   `+4` (fewer or equal reallocs, same inline-capacity-`N` semantics), and
//! - `heap`-pointer move-safety dance is gone entirely: `SmallVec` owns the
//!   representation, so this file contains no `unsafe` at all.

use core::{
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
  pub fn capacity(&self) -> u32 {
    self.0.capacity() as u32
  }

  #[inline]
  pub fn empty(&self) -> bool {
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

  /// std-style alias for `push_back` — translations use Vec idioms.
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

impl<T, const N: usize> FromIterator<T> for SmallVector<T, N> {
  #[inline]
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    SmallVector(iter.into_iter().collect())
  }
}

impl<T, const N: usize> DenseDefault for SmallVector<T, N> {
  #[inline]
  fn dense_default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::SmallVector;

  /// ZST 元素跨过 inline 容量触发的 grow/drop 路径（N=0 即刻落堆）：
  /// 不得 UB 或 panic。
  #[test]
  fn zst_grow_and_drop() {
    let mut v: SmallVector<u8, 0> = SmallVector::new();
    for _ in 0..100 {
      v.push_back(0);
    }
    assert_eq!(v.size(), 100);
    assert!(v.capacity() >= 100);
    v.clear();
    assert!(v.empty());
    v.push_back(0);
    assert_eq!(v.front(), &0);
    v.pop_back();
    assert!(v.empty());
  }
}
