use ulua_vm::{
  functions::{lua_isnumber::lua_isnumber, lua_touserdatatagged::lua_touserdatatagged},
  macros::{lua_l_typeerror::luaL_typeerror, lua_tointeger::lua_tointeger},
  records::lua_state::lua_State,
};

use crate::common::functions::k_int_64_tag::K_INT_64_TAG;

pub(crate) fn get_int_64(l: *mut lua_State, idx: i32) -> i64 {
  unsafe {
    let p = lua_touserdatatagged(l, idx, K_INT_64_TAG);
    if !p.is_null() {
      return *(p as *const i64);
    }

    if lua_isnumber(l, idx) != 0 {
      return lua_tointeger!(l, idx) as i64;
    }

    luaL_typeerror!(l, 1, "int64");
  }
}
