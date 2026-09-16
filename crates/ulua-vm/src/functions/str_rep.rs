//! Node: `cxx:Function:Luau.VM:VM/src/lstrlib.cpp:83:str_rep`
//!
//! `string.rep` — repeat the argument `n` times into a single buffer, doubling
//! the already-written prefix each step so the fill is O(result) with log(n)
//! memcpys (the classic exponential-pattern trick), with an overflow guard.

use core::{
  ffi::c_int,
  ptr::{copy_nonoverlapping, null_mut},
};

use crate::{
  functions::{
    lua_l_buffinitsize::lua_l_buffinitsize, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checklstring::lua_l_checklstring, lua_l_pushresultsize::lua_l_pushresultsize,
    lua_pushlstring::lua_pushlstring,
  },
  macros::{lua_l_error::luaL_error, maxssize::MAXSSIZE},
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn str_rep(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let n = lua_l_checkinteger(l, 2);

    if n <= 0 {
      lua_pushlstring(l, c"".as_ptr(), 0);
      return 1;
    }

    if len > (MAXSSIZE as usize) / (n as usize) {
      luaL_error!(l, "resulting string too large");
    }

    let total = len * (n as usize);

    let mut b: LuaLStrbuf = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };
    let mut ptr = lua_l_buffinitsize(l, &mut b, total);

    let start = ptr;
    let mut left = total;
    let mut step = len;

    copy_nonoverlapping(s, ptr, len);
    ptr = ptr.add(len);
    left -= len;

    // use the increasing 'pattern' inside our target buffer to fill the next part
    while step < left {
      copy_nonoverlapping(start, ptr, step);
      ptr = ptr.add(step);
      left -= step;
      step <<= 1;
    }

    // fill tail
    copy_nonoverlapping(start, ptr, left);

    lua_l_pushresultsize(&mut b, total);

    1
  }
}
