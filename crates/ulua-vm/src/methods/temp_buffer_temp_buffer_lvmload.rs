use core::{mem::size_of, ptr::null_mut};

use crate::{functions::lua_m_free::lua_m_free, records::temp_buffer::TempBuffer};

impl<T> Default for TempBuffer<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> TempBuffer<T> {
  pub fn new() -> Self {
    Self {
      l: null_mut(),
      data: null_mut(),
      count: 0,
    }
  }
}

impl<T> Drop for TempBuffer<T> {
  fn drop(&mut self) {
    if !self.data.is_null() {
      // Safety: data 只会来自 allocate（经 l 的分配器按 count*size_of::<T>() 申请且未被别处释放），
      // 以同一 l、同一字节数归还给 lua_m_free
      unsafe { lua_m_free(self.l, self.data as *mut u8, self.count * size_of::<T>(), 0) };
      self.data = null_mut();
      self.count = 0;
    }
  }
}
