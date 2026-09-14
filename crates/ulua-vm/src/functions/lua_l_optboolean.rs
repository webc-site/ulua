use core::ffi::c_int;

use crate::{
  functions::lua_l_checkboolean::lua_l_checkboolean, macros::lua_l_opt::luaL_opt,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_optboolean(l: *mut lua_State, narg: c_int, def: bool) -> bool {
  unsafe {
    let def_cint = if def { 1 } else { 0 };
    luaL_opt!(l, lua_l_checkboolean, narg, def_cint) != 0
  }
}
