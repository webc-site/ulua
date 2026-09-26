//! `luaL_addstring` (VM/include/lualib.h macro) — append a NUL-terminated C
//! string to a `luaL_Strbuf` via `luaL_addlstring` with `strlen(s)`.

use core::ffi::c_char;

use crate::{
  functions::{cstr_bytes, lua_l_addlstring::lua_l_addlstring},
  records::lua_l_strbuf::LuaLStrbuf,
};

/// # Safety
///
/// `s` 必须指向以 NUL 结尾的存活字节串（strlen 扫描定读取界）；`b` 必须指向已在存活
/// `lua_State` 上初始化、尚未 `pushresult` 的 `LuaLStrbuf`，追加经 `lua_l_addlstring`
/// 的扩容契约完成。cpp lualib.h:107 `luaL_addstring`。
pub(crate) unsafe fn lua_l_addstring(b: &mut LuaLStrbuf, s: *const c_char) {
  // Safety: 契约保证 s 至首个 NUL 前可读，该段字节交给 addlstring 界内追加
  unsafe {
    lua_l_addlstring(b, cstr_bytes(s));
  }
}
