//! Node: `cxx:Function:Luau.VM:VM/src/lstrlib.cpp:71:str_upper`
//!
//! `string.upper` — uppercase each byte of the argument into a fresh buffer.

use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
  slice,
};

use crate::{
  functions::{
    lua_l_buffinitsize::lua_l_buffinitsize, lua_l_checklstring::lua_l_checklstring,
    lua_l_pushresultsize::lua_l_pushresultsize,
  },
  macros::uchar::uchar,
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn str_upper(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);

    let mut b: LuaLStrbuf = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };
    let ptr = lua_l_buffinitsize(l, &mut b, len);

    // 源与目标切片 zip 单次遍历，消除索引与越界检查
    let src = slice::from_raw_parts(s, len);
    for (d, b) in slice::from_raw_parts_mut(ptr, len).iter_mut().zip(src) {
      *d = (uchar(*b as c_int) as u8).to_ascii_uppercase() as c_char;
    }

    lua_l_pushresultsize(&mut b, len);
    1
  }
}
