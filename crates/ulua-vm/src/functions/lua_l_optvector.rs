use core::ffi::c_int;

use crate::{
  functions::lua_l_checkvector::lua_l_checkvector, macros::lua_l_opt::luaL_opt,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_l_optvector(
  l: *mut lua_State,
  narg: c_int,
  def: *const f32,
) -> *const f32 {
  unsafe { luaL_opt!(l, lua_l_checkvector, narg, def) }
}
