use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  pub fn push_back(&mut self, item: T) {
    let storage = unsafe { &mut *self.storage };
    ulua_common::LUAU_ASSERT!(storage.len() == self.offset + self.size_);
    storage.push(item);
    self.size_ += 1;
  }
}
