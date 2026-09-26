//! Faithful port of Luau `TempVector<T>` (`Ast/include/Luau/Parser.h:20-52`) —
//! a window `storage[offset .. offset + size_]` over a parser scratch `vector`.
//! C++ holds `std::vector<T>& storage`; the Rust analog keeps the same borrow
//! through a `*mut Vec<T>` plus `PhantomData<&'a mut T>`, so every read goes
//! through [`TempVector::as_slice`] (bounds-checked) instead of raw offsets.

use alloc::vec::Vec;
use core::{
  fmt::{Debug, Formatter, Result},
  marker::PhantomData,
  ops::{Deref, DerefMut, Index, IndexMut},
  slice::{Iter, IterMut, SliceIndex},
};

pub struct TempVector<'a, T> {
  pub(crate) storage: *mut Vec<T>,
  pub(crate) offset: usize,
  pub(crate) size_: usize,
  pub(crate) _marker: PhantomData<&'a mut T>,
}

impl<'a, T> TempVector<'a, T> {
  /// 本视图独占的存储区间 `[offset, offset + size_)`，对应 cpp 的
  /// `storage[offset + i]`。
  ///
  /// # Safety note
  /// `storage` 由 [`TempVector::new`] 从 `&mut Vec<T>` 取得，`'a` 内不离开该
  /// 借用；不变式（`push_back` 与 `Drop` 双向断言）`size_ == storage.len() -
  /// offset` 保证区间恒在 Vec 长度内，故切片索引越界只会是调用方 bug 触发的
  /// panic，而非未定义行为。
  #[inline]
  pub fn as_slice(&self) -> &[T] {
    // Safety: 见上方 Safety note。
    let storage = unsafe { &*self.storage };
    &storage[self.offset..self.offset + self.size_]
  }

  /// 本视图独占的存储区间 `[offset, offset + size_)` 的可变切片。
  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    // Safety: 见上方 Safety note，且 self 持有独占可变借用。
    let storage = unsafe { &mut *self.storage };
    &mut storage[self.offset..self.offset + self.size_]
  }
}

impl<'a, T> Deref for TempVector<'a, T> {
  type Target = [T];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl<'a, T> DerefMut for TempVector<'a, T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_slice()
  }
}

impl<'a, T, I> Index<I> for TempVector<'a, T>
where
  I: SliceIndex<[T]>,
{
  type Output = I::Output;

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    &self.as_slice()[index]
  }
}

impl<'a, T, I> IndexMut<I> for TempVector<'a, T>
where
  I: SliceIndex<[T]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    &mut self.as_mut_slice()[index]
  }
}

impl<'b, 'a, T> IntoIterator for &'b TempVector<'a, T> {
  type Item = &'b T;
  type IntoIter = Iter<'b, T>;

  #[inline(always)]
  fn into_iter(self) -> Self::IntoIter {
    self.as_slice().iter()
  }
}

impl<'b, 'a, T> IntoIterator for &'b mut TempVector<'a, T> {
  type Item = &'b mut T;
  type IntoIter = IterMut<'b, T>;

  #[inline(always)]
  fn into_iter(self) -> Self::IntoIter {
    self.as_mut_slice().iter_mut()
  }
}

impl<'a, T> Debug for TempVector<'a, T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let storage_len = unsafe {
      // Safety: storage 由 TempVector::new 从 &mut Vec<T> 取得，PhantomData<&'a mut T> 记录该借用：非空、对齐且在 'a 内存活；len() 仅只读长度字段，Debug 期间单线程无并发变更，读区间不变式 size_==len-offset 由 push_back/Drop 双向断言维持。
      (*self.storage).len()
    };
    f.debug_struct("TempVector")
      .field("storage_len", &storage_len)
      .field("offset", &self.offset)
      .field("size_", &self.size_)
      .finish()
  }
}
