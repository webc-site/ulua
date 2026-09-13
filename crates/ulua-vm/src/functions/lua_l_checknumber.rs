use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tonumberx::lua_tonumberx, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_checknumber(l: *mut lua_State, narg: c_int) -> f64 {
  unsafe {
    let mut isnum: i32 = 0;
    let d = lua_tonumberx(l, narg, &mut isnum);
    if isnum == 0 {
      tag_error(l, narg, LuaType::Number as c_int);
    }
    d
  }
}

// lualib.h name
pub use lua_l_checknumber as luaL_checknumber;
