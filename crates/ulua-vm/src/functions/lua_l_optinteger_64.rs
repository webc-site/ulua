use core::ffi::c_int;

use crate::{
  functions::lua_l_checkinteger_64::lua_l_checkinteger_64, macros::lua_l_opt::luaL_opt,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_l_optinteger_64")]
pub(crate) unsafe fn lua_l_optinteger_64(l: *mut lua_State, narg: c_int, def: i64) -> i64 {
  unsafe {
    // The macro luaL_opt! expands to:
    // if lua_isnoneornil(l, narg) { def } else { luaL_checkinteger64(l, narg) }
    // We use the already-translated lua_l_checkinteger_64 as the function argument.
    luaL_opt!(l, lua_l_checkinteger_64, narg, def)
  }
}
