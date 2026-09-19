use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  /// cpp `TempVector::back()`：`LUAU_ASSERT(size_ > 0)` 后
  /// `return storage[offset + size_ - 1]`；切片索引取代裸偏移解引用。
  pub fn back(&self) -> &T {
    ulua_common::LUAU_ASSERT!(self.size_ > 0);
    let values = self.as_slice();
    &values[values.len() - 1]
  }
}
