use crate::records::temp_vector::TempVector;

impl<'a, T> Drop for TempVector<'a, T> {
  fn drop(&mut self) {
    // SAFETY: storage 指向构造时借用的 Vec，借用仍存活。
    let storage = unsafe { &mut *self.storage };
    ulua_common::LUAU_ASSERT!(storage.len() == self.offset + self.size_);
    storage.truncate(self.offset);
  }
}
