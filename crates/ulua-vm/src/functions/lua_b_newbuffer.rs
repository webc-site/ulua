use core::ptr::write_bytes;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::luaM_newgco_, lua_m_toobig::lua_m_toobig},
  macros::{lua_c_init::luaC_init, max_buffer_size::MAX_BUFFER_SIZE, sizebuffer::sizebuffer},
  type_aliases::{buffer::Buffer, lua_state::lua_State},
};

pub(crate) unsafe fn lua_b_newbuffer(l: *mut lua_State, s: usize) -> *mut Buffer {
  unsafe {
    if s > MAX_BUFFER_SIZE as usize {
      lua_m_toobig(l);
    }

    let b = luaM_newgco_(l, sizebuffer(s), (*l).activememcat) as *mut Buffer;
    luaC_init!(l, b, LuaType::Buffer as i32);
    (*b).len = s as u32;
    write_bytes((*b).data.as_mut_ptr(), 0, (*b).len as usize);
    b
  }
}
