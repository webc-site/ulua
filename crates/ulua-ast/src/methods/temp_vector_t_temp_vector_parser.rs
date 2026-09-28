use alloc::vec::Vec;
use core::{marker::PhantomData, ptr::NonNull};

use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  pub fn new(storage: &mut Vec<T>) -> Self {
    let offset = storage.len();
    Self {
      // NonNull::from(&mut Vec) 即「非空 + 'a 内存活」的构造端证明（引用不可为空）。
      storage: NonNull::from(storage),
      offset,
      size_: 0,
      _marker: PhantomData,
    }
  }
}

impl<'a, T> Drop for TempVector<'a, T> {
  fn drop(&mut self) {
    // Safety: storage 指向构造时借用的 Vec，借用仍存活且本视图是唯一写者。
    let storage = unsafe { self.storage.as_mut() };
    ulua_common::LUAU_ASSERT!(storage.len() == self.offset + self.size_);
    storage.truncate(self.offset);
  }
}
