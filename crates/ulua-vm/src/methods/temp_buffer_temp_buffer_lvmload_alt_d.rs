use core::{mem::size_of, ptr::null_mut};

use crate::{functions::lua_m_free::luaM_free_, records::temp_buffer::TempBuffer};

impl<T> Drop for TempBuffer<T> {
  fn drop(&mut self) {
    if !self.data.is_null() {
      unsafe { luaM_free_(self.l, self.data as *mut u8, self.count * size_of::<T>(), 0) };
      self.data = null_mut();
      self.count = 0;
    }
  }
}
