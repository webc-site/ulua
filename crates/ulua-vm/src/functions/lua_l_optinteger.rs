use core::ffi::c_int;

use crate::{
  functions::lua_l_checkinteger::lua_l_checkinteger, macros::lua_l_opt::luaL_opt,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_optinteger(l: *mut lua_State, narg: c_int, def: c_int) -> c_int {
  unsafe { luaL_opt!(l, lua_l_checkinteger, narg, def) }
}
