//! Faithful port of Luau `TempVector<T>` (`Ast/include/Luau/Parser.h:20-52`) —
//! a window `storage[offset .. offset + size_]` over a parser scratch `vector`.
//! C++ holds `std::vector<T>& storage`; the Rust analog keeps the same borrow
//! through a `*mut Vec<T>` plus `PhantomData<&'a mut T>`, so every read goes
//! through [`TempVector::as_slice`] (bounds-checked) instead of raw offsets.

use alloc::vec::Vec;
use core::{
  fmt::{Debug, Formatter, Result},
  marker::PhantomData,
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
  pub(crate) fn as_slice(&self) -> &[T] {
    // SAFETY: 见上方 Safety note。
    let storage = unsafe { &*self.storage };
    &storage[self.offset..self.offset + self.size_]
  }
}

impl<'a, T> Debug for TempVector<'a, T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let storage_len = unsafe { (*self.storage).len() };
    f.debug_struct("TempVector")
      .field("storage_len", &storage_len)
      .field("offset", &self.offset)
      .field("size_", &self.size_)
      .finish()
  }
}
