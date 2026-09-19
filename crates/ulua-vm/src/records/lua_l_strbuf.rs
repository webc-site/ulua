//! Source: `VM/include/lualib.h` (lualib.h:86-98, hand-ported)

use core::{ffi::c_char, ptr::null_mut};

use crate::{records::t_string::tstring, type_aliases::lua_state::lua_State};

// luaconf.h:96
pub const LUA_BUFFERSIZE: usize = 512;

#[repr(C)]
#[derive(Debug)]
pub struct LuaLStrbuf {
  pub p: *mut c_char,   // current position in buffer
  pub end: *mut c_char, // end of the current buffer
  pub l: *mut lua_State,
  pub storage: *mut tstring,
  pub buffer: [c_char; LUA_BUFFERSIZE],
}

impl LuaLStrbuf {
  /// 空缓冲：字段由 `luaL_buffinit` / `luaL_buffinitsize` 填充。
  pub fn new() -> Self {
    Self {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    }
  }
}
