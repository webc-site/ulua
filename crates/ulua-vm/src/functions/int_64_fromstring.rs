use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_optinteger::lua_l_optinteger, lua_o_str_2_l::lua_o_str_2_l,
    lua_pushinteger_64::lua_pushinteger_64, lua_pushnil::lua_pushnil,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_fromstring"))]
pub(crate) unsafe extern "C-unwind" fn int64_fromstring(l: *mut lua_State) -> c_int {
  unsafe {
    let s = luaL_checkstring!(l, 1);
    let base = lua_l_optinteger(l, 2, 10);
    luaL_argcheck!(
      l,
      (2 <= base as i32) && (base as i32 <= 36),
      2,
      "base out of range"
    );

    match lua_o_str_2_l(s, base as i32) {
      Some(result) => lua_pushinteger_64(l, result),
      None => lua_pushnil(l),
    }

    1
  }
}
