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
