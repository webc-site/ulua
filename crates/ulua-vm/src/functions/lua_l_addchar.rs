use core::ffi::c_char;

use crate::{functions::lua_l_prepbuffsize::lua_l_prepbuffsize, records::lua_l_strbuf::LuaLStrbuf};

/// # Safety
///
/// `b` 必须指向已在存活 `lua_State` 上经 `lua_l_buffinit`/`lua_l_buffinitsize` 初始化、
/// 尚未 `pushresult` 的 `LuaLStrbuf`（`p`/`end` 为其缓冲内可写游标）；缓冲写满时先经
/// `lua_l_prepbuffsize` 扩容（GC 分配、可抛错并重挂 `b.p`），单字节回写只落在扩容后的
/// 可写区。cpp lualib.h:106 `luaL_addchar`。
pub(crate) unsafe fn lua_l_addchar(b: &mut LuaLStrbuf, c: c_char) {
  // Safety: 契约保证 b 的 p/end 为其缓冲界内游标；prep 扩容后 p 已重挂，单字节写入不越出可写区
  unsafe {
    if !(b.p < b.end) {
      lua_l_prepbuffsize(b as *mut _, 1);
    }
    *b.p = c;
    b.p = b.p.add(1);
  }
}
