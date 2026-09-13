use core::ffi::{CStr, c_char};

use crate::{
  functions::lua_s_newlstr::luaS_newlstr,
  records::{lua_state::lua_State, t_string::tstring},
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
/// `s` must be a valid pointer to a null-terminated C string.
pub unsafe fn lua_s_new(l: *mut lua_State, s: *const c_char) -> *mut tstring {
  unsafe {
    let len = CStr::from_ptr(s).to_bytes().len();
    luaS_newlstr(l, s, len)
  }
}

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
/// `s` must be a valid pointer to a null-terminated C string.
pub unsafe fn lua_s_newliteral(l: *mut lua_State, s: *const c_char) -> *mut tstring {
  unsafe {
    let len = CStr::from_ptr(s).to_bytes().len();
    luaS_newlstr(l, s, len)
  }
}

pub use lua_s_new as luaS_new;
pub use lua_s_newliteral as luaS_newliteral;
