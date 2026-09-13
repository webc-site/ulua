use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use crate::{
  functions::{
    c_slice, c_slice_mut, lua_l_buffinitsize::lua_l_buffinitsize,
    lua_l_checklstring::lua_l_checklstring, lua_l_pushresultsize::lua_l_pushresultsize,
  },
  macros::uchar::uchar,
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn str_lower(l: *mut lua_State) -> c_int {
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

    // SAFETY：ptr/s 均指向 len 字节的可写/可读缓冲（buffinitsize 按 len 分配）。
    for (dst, &src) in c_slice_mut(ptr, len).iter_mut().zip(c_slice(s, len)) {
      *dst = (uchar(src as c_int) as u8).to_ascii_lowercase() as c_char;
    }

    lua_l_pushresultsize(&mut b, len);
    1
  }
}
