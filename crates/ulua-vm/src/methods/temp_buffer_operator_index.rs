use core::ops::{Index, IndexMut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::temp_buffer::TempBuffer;

impl<T> TempBuffer<T> {
  pub fn operator_index(&self, index: usize) -> &T {
    LUAU_ASSERT!(index < self.count);
    unsafe { &*self.data.add(index) }
  }

  pub fn operator_index_mut(&mut self, index: usize) -> &mut T {
    LUAU_ASSERT!(index < self.count);
    unsafe { &mut *self.data.add(index) }
  }
}

impl<T> Index<usize> for TempBuffer<T> {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    self.operator_index(index)
  }
}

impl<T> IndexMut<usize> for TempBuffer<T> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.operator_index_mut(index)
  }
}
