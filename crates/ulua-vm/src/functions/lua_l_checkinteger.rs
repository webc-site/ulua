use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tointegerx::lua_tointegerx, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_checkinteger(l: *mut lua_State, narg: c_int) -> c_int {
  unsafe {
    let mut isnum: i32 = 0;

    let d = lua_tointegerx(l, narg, &mut isnum);

    if isnum == 0 {
      tag_error(l, narg, LuaType::Number as c_int);
    }

    d
  }
}

// lualib.h name
pub use lua_l_checkinteger as luaL_checkinteger;
