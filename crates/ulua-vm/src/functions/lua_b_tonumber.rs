use core::{ffi::c_char, ptr::null_mut};

use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_optinteger::lua_l_optinteger, lua_pushnil::lua_pushnil,
    lua_pushnumber::lua_pushnumber, lua_tonumberx::lua_tonumberx,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  type_aliases::lua_state::lua_State,
};

// Helper for isspace: check if a u8 value corresponds to an ASCII whitespace character
#[inline]
fn isspace(c: u8) -> bool {
  matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

// Helper function for strtoull-like behavior via libc-compatible symbol
unsafe fn strtoull(s: *const c_char, endptr: &mut *mut c_char, base: u32) -> u64 {
  unsafe {
    unsafe extern "C" {
      fn strtoull(s: *const c_char, endptr: *mut *mut c_char, base: u32) -> u64;
    }
    strtoull(s, endptr as *mut *mut c_char, base)
  }
}

pub(crate) unsafe extern "C-unwind" fn lua_b_tonumber(l: *mut lua_State) -> i32 {
  unsafe {
    let base = lua_l_optinteger(l, 2, 10);

    if base == 10 {
      // standard conversion
      let mut isnum: i32 = 0;
      let n = lua_tonumberx(l, 1, &mut isnum);
      if isnum != 0 {
        lua_pushnumber(l, n);
        return 1;
      }
      lua_l_checkany(l, 1); // error if we don't have any argument
    } else {
      let s1 = luaL_checkstring!(l, 1);
      luaL_argcheck!(l, (2..=36).contains(&base), 2, "base out of range");

      let mut s2: *mut c_char = null_mut();
      let n = strtoull(s1, &mut s2, base as u32);

      if s1 != s2 {
        // at least one valid digit?
        while isspace(*s2 as u8) {
          s2 = s2.add(1);
        } // skip trailing spaces

        if *s2 == b'\0' as c_char {
          // no invalid trailing characters?
          lua_pushnumber(l, n as f64);
          return 1;
        }
      }
    }

    lua_pushnil(l); // else not a number
    1
  }
}
