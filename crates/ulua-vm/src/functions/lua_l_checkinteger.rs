use core::{
  ffi::{c_int, c_void},
  mem::transmute,
};

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

    // The dependency card for lua_tointegerx shows an empty signature in the snippet,
    // but the C++ source and the logic of this function require it to take 3 arguments
    // and return an int. We must call it with the arguments required by the logic.
    let d = {
      let func: unsafe fn(*mut lua_State, c_int, *mut c_int) -> c_int =
        transmute(lua_tointegerx as *const c_void);
      func(l, narg, &mut isnum)
    };

    if isnum == 0 {
      tag_error(l, narg, LuaType::Number as c_int);
    }

    d
  }
}

// lualib.h name
pub use lua_l_checkinteger as luaL_checkinteger;
