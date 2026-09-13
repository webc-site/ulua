use core::ffi::{c_char, c_int};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger::lua_l_checkinteger,
    lua_pushinteger::lua_pushinteger, lua_type::lua_type,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_tostring::lua_tostring},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn lua_b_select(l: *mut lua_State) -> c_int {
  unsafe {
    let n = lua_gettop(l);
    let first_type = lua_type(l, 1);
    if first_type == LuaType::String as i32 {
      let str_ptr = lua_tostring!(l, 1);
      let first_char = *str_ptr;
      if first_char == b'#' as c_char {
        lua_pushinteger(l, n - 1);
        return 1;
      }
    }

    let i = lua_l_checkinteger(l, 1);
    let i = if i < 0 {
      n + i
    } else if i > n {
      n
    } else {
      i
    };

    luaL_argcheck!(l, 1 <= i, 1, "index out of range");
    n - i
  }
}
