use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::lua_m_newarray::luaM_newarray, records::temp_buffer::TempBuffer,
  type_aliases::lua_state::lua_State,
};

impl<T> TempBuffer<T> {
  pub(crate) unsafe fn allocate(&mut self, l: *mut lua_State, count: usize) {
    unsafe {
      LUAU_ASSERT!(self.l.is_null());
      self.l = l;
      self.data = luaM_newarray!(l, count, T, 0);
      self.count = count;
    }
  }
}
