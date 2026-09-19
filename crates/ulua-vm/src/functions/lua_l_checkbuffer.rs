use core::ffi::{c_int, c_void};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tobuffer::lua_tobuffer, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_checkbuffer(l: *mut lua_State, narg: c_int, len: *mut usize) -> *mut c_void {
  unsafe {
    let b = lua_tobuffer(l, narg, len);

    if b.is_null() {
      tag_error(l, narg, LuaType::Buffer as c_int);
    }

    b
  }
}

// lualib.h name
