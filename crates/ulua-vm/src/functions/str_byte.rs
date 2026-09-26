use core::slice;

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_checkstack::lua_l_checkstack,
    lua_l_optinteger::lua_l_optinteger, lua_pushinteger::lua_pushinteger, posrelat::posrelat,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, uchar::uchar},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn str_byte(l: *mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let mut posi = posrelat(lua_l_optinteger(l, 2, 1), len);
    let mut pose = posrelat(lua_l_optinteger(l, 3, posi), len);

    if posi <= 0 {
      posi = 1;
    }
    if (pose as usize) > len {
      pose = len as i32;
    }

    if posi > pose {
      return 0; // empty interval; return no values
    }

    let n = pose - posi + 1;
    // oracle 同位死守卫（lstrlib.cpp:137 `if (n <= 0)`）：posi/pose 已钳位后
    // 恒假——忠实保留，防 sync-cpp 时漂移
    if posi + n <= pose {
      // overflow?
      luaL_error!(l, "string slice too long");
    }

    lua_l_checkstack(l, n, "string slice too long");

    let s_ptr = s.add((posi - 1) as usize);
    // 源切片单次遍历，消除索引与越界检查
    for &b in slice::from_raw_parts(s_ptr, n as usize) {
      lua_pushinteger(l, uchar(b as i32) as i32);
    }

    n
  }
}

lua_lib_fn!(pub fn str_byte, str_byte_arm);
