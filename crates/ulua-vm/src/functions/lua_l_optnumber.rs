use core::ffi::c_int;

use crate::{
  functions::lua_l_checknumber::lua_l_checknumber, macros::lua_l_opt::luaL_opt,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_optnumber(l: *mut lua_State, narg: c_int, def: f64) -> f64 {
  unsafe { luaL_opt!(l, lua_l_checknumber, narg, def) }
}
