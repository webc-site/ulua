use core::{
  ffi::{c_int, c_void},
  mem::transmute,
};

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
    // The dependency card for lua_tobuffer shows an empty signature in the snippet,
    // but the C++ source and the logic of this function require it to take 3 arguments
    // and return a pointer. We must call it with the arguments required by the logic.
    // We use a transmute or a cast if necessary to satisfy the compiler if the stub
    // signature is truly empty, but here we follow the C++ signature.
    let b = {
      let func: unsafe fn(*mut lua_State, c_int, *mut usize) -> *mut c_void =
        transmute(lua_tobuffer as *const c_void);
      func(l, narg, len)
    };

    if b.is_null() {
      tag_error(l, narg, LuaType::Buffer as c_int);
    }

    b
  }
}

// lualib.h name
pub use lua_l_checkbuffer as luaL_checkbuffer;
