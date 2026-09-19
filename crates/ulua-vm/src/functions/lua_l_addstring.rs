//! `luaL_addstring` (VM/include/lualib.h macro) — append a NUL-terminated C
//! string to a `luaL_Strbuf` via `luaL_addlstring` with `strlen(s)`.

use core::ffi::{CStr, c_char};

use crate::{functions::lua_l_addlstring::lua_l_addlstring, records::lua_l_strbuf::LuaLStrbuf};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_l_addstring(b: *mut LuaLStrbuf, s: *const c_char) {
  unsafe {
    let len = CStr::from_ptr(s).to_bytes().len();
    lua_l_addlstring(b, s, len);
  }
}
