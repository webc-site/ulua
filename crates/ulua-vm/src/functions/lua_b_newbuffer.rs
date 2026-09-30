use core::ptr::write_bytes;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, lua_m_toobig::lua_m_toobig},
  macros::{lua_c_init::luaC_init, max_buffer_size::MAX_BUFFER_SIZE, sizebuffer::sizebuffer},
  records::{lua_state::LuaState, luau_buffer::LuauBuffer as Buffer},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_b_newbuffer(l: *mut LuaState, s: usize) -> *mut Buffer {
  unsafe {
    if s > MAX_BUFFER_SIZE as usize {
      lua_m_toobig(l);
    }

    let b = lua_m_newgco(l, sizebuffer(s), (*l).activememcat) as *mut Buffer;
    luaC_init!(l, b, LuaType::Buffer as i32);
    (*b).len = s as u32;
    write_bytes((*b).data.as_mut_ptr(), 0, (*b).len as usize);
    b
  }
}
