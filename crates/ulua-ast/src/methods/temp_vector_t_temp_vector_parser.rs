use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  pub fn new(storage: &mut Vec<T>) -> Self {
    let offset = storage.len();
    Self {
      storage: storage as *mut _,
      offset,
      size_: 0,
      _marker: PhantomData,
    }
  }
}

impl<'a, T> Drop for TempVector<'a, T> {
  fn drop(&mut self) {
    // Safety: storage 指向构造时借用的 Vec，借用仍存活。
    let storage = unsafe { &mut *self.storage };
    ulua_common::LUAU_ASSERT!(storage.len() == self.offset + self.size_);
    storage.truncate(self.offset);
  }
}
