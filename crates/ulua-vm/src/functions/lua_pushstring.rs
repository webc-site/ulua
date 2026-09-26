use core::ffi::c_char;

use crate::{
  functions::{cstr_bytes, lua_pushlstring::lua_pushlstring, lua_pushnil::lua_pushnil},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`s` 允许 NULL（走 `lua_pushnil` 分支），非空时须指向以 NUL 结尾的合法 C 串
/// （`cstr_bytes` 扫描定长后交 `lua_pushlstring` 复制）；两分支都会向 `(*l).top` 压 1 个值，`lua_pushlstring` 可分配并触发 GC。
/// cpp VM/src/lapi.cpp:754
pub unsafe fn lua_pushstring(l: *mut LuaState, s: *const c_char) {
  unsafe {
    if s.is_null() {
      lua_pushnil(l);
    } else {
      // strlen 等价：cstr_bytes 零拷贝扫描至 NUL
      let len = cstr_bytes(s).len();
      lua_pushlstring(l, s, len);
    }
  }
}
