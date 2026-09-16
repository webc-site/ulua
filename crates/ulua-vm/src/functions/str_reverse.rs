//! Node: `cxx:Function:Luau.VM:VM/src/lstrlib.cpp:47:str_reverse`
//!
//! `string.reverse` — copy the argument bytes into a fresh buffer back-to-front.

use core::{ffi::c_int, ptr::null_mut, slice};

use crate::{
  functions::{
    lua_l_buffinitsize::lua_l_buffinitsize, lua_l_checklstring::lua_l_checklstring,
    lua_l_pushresultsize::lua_l_pushresultsize,
  },
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn str_reverse(l: *mut lua_State) -> c_int {
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

    // 源逆序 zip 目标切片，单次遍历消除索引与越界检查
    let src = slice::from_raw_parts(s, len);
    for (d, r) in slice::from_raw_parts_mut(ptr, len)
      .iter_mut()
      .zip(src.iter().rev())
    {
      *d = *r;
    }

    lua_l_pushresultsize(&mut b, len);
    1
  }
}
