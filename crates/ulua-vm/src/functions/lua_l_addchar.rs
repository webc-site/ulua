//! `luaL_addchar` (VM/include/lualib.h macro) — append one byte to a
//! `luaL_Strbuf`, growing it first if the inline/current buffer is full.

use core::ffi::c_char;

use crate::{functions::lua_l_prepbuffsize::lua_l_prepbuffsize, records::lua_l_strbuf::LuaLStrbuf};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_l_addchar(b: *mut LuaLStrbuf, c: c_char) {
  unsafe {
    if !((*b).p < (*b).end) {
      lua_l_prepbuffsize(b, 1);
    }
    *(*b).p = c;
    (*b).p = (*b).p.add(1);
  }
}
