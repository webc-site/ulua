use core::ptr::write_bytes;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_m_newgco::lua_m_newgco, lua_m_toobig::lua_m_toobig},
  macros::{lua_c_init::luaC_init, max_buffer_size::MAX_BUFFER_SIZE, sizebuffer::sizebuffer},
  records::{lua_state::LuaState, luau_buffer::LuauBuffer as Buffer},
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；体内 `lua_m_toobig`/`lua_m_newgco`/
/// `luaC_init!` 仍收裸形，转手各经一次 `l.as_mut_ptr()` 就地重建（借用窗止于当句），`l.activememcat`
/// 场域读数与返回的 `b` 裸指针存活前提均由调用方给出，屏障按 r16-v21 判例保留。
pub(crate) unsafe fn lua_b_newbuffer(l: &mut LuaState, s: usize) -> *mut Buffer {
  unsafe {
    if s > MAX_BUFFER_SIZE as usize {
      lua_m_toobig(l.as_mut_ptr());
    }

    let b = lua_m_newgco(l.as_mut_ptr(), sizebuffer(s), l.activememcat) as *mut Buffer;
    luaC_init!(l.as_mut_ptr(), b, LuaType::Buffer as i32);
    (*b).len = s as u32;
    write_bytes((*b).data.as_mut_ptr(), 0, (*b).len as usize);
    b
  }
}
