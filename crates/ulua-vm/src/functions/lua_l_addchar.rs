//! `luaL_addchar` (VM/include/lualib.h macro) — append one byte to a
//! `luaL_Strbuf`, growing it first if the inline/current buffer is full.

use core::ffi::c_char;

use crate::{functions::lua_l_prepbuffsize::lua_l_prepbuffsize, records::lua_l_strbuf::LuaLStrbuf};

pub(crate) unsafe fn lua_l_addchar(b: *mut LuaLStrbuf, c: c_char) {
  unsafe {
    if !((*b).p < (*b).end) {
      lua_l_prepbuffsize(b, 1);
    }
    *(*b).p = c;
    (*b).p = (*b).p.add(1);
  }
}
