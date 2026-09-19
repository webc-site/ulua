use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  /// cpp `TempVector::front()`：`LUAU_ASSERT(size_ > 0)` 后 `return storage[offset]`。
  /// 切片索引使越界在 release 下也 panic，不再裸偏移解引用未初始化/越界内存。
  pub fn front(&self) -> &T {
    ulua_common::LUAU_ASSERT!(self.size_ > 0);
    &self.as_slice()[0]
  }
}
