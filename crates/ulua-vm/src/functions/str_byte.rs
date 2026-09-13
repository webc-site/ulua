use core::{ffi::c_int, slice};

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_checkstack::lua_l_checkstack,
    lua_l_optinteger::lua_l_optinteger, lua_pushinteger::lua_pushinteger, posrelat::posrelat,
  },
  macros::{lua_l_error::luaL_error, uchar::uchar},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_str_byte")]
pub(crate) unsafe extern "C-unwind" fn str_byte(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let mut posi = posrelat(lua_l_optinteger(l, 2, 1), len);
    let mut pose = posrelat(lua_l_optinteger(l, 3, posi), len);

    if posi <= 0 {
      posi = 1;
    }
    if (pose as usize) > len {
      pose = len as c_int;
    }

    if posi > pose {
      return 0; // empty interval; return no values
    }

    let n = pose - posi + 1;
    if posi + n <= pose {
      // overflow?
      luaL_error!(l, "string slice too long");
    }

    lua_l_checkstack(l, n, "string slice too long");

    let s_ptr = s.add((posi - 1) as usize);
    // 源切片单次遍历，消除索引与越界检查
    for &b in slice::from_raw_parts(s_ptr, n as usize) {
      lua_pushinteger(l, uchar(b as c_int) as c_int);
    }

    n
  }
}
