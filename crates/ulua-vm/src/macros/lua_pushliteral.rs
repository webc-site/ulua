use core::{
  ffi::{CStr, c_char, c_void},
  mem::transmute,
};

use crate::functions::lua_pushlstring::lua_pushlstring;

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
/// `s` must be a valid pointer to a null-terminated C string.
#[inline(always)]
pub unsafe fn lua_pushliteral(l: *mut c_void, s: *const c_char) -> *mut c_void {
  unsafe {
    let func: unsafe extern "C-unwind" fn(*mut c_void, *const c_char, usize) -> *mut c_void =
      transmute(lua_pushlstring as *const c_void);

    let len = CStr::from_ptr(s).to_bytes().len();
    func(l, s, len)
  }
}

pub use lua_pushliteral as LUA_PUSHLITERAL;
