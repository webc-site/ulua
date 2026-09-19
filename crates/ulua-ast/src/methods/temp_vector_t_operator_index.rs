use core::ops::Index;

use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  /// cpp `TempVector::operator[](index)`：`LUAU_ASSERT(index < size_)` 后
  /// `return storage[offset + index]`。切片索引替裸偏移解引用，越界在 release
  /// 下也 panic（可被 parser 的 catch_unwind 边界接住），debug 仍由断言早失败。
  pub fn operator_index(&self, index: usize) -> &T {
    ulua_common::LUAU_ASSERT!(index < self.size_);
    &self.as_slice()[index]
  }
}

impl<'a, T> Index<usize> for TempVector<'a, T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    self.operator_index(index)
  }
}
