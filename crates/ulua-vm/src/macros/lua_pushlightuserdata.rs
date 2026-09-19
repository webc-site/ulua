use core::{ffi::c_void, mem::transmute};

use crate::functions::lua_pushlightuserdatatagged::lua_pushlightuserdatatagged;

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
#[inline(always)]
pub unsafe fn lua_pushlightuserdata(l: *mut c_void, p: *mut c_void) {
  unsafe {
    let func: unsafe fn(*mut c_void, *mut c_void, i32) =
      transmute(lua_pushlightuserdatatagged as *const c_void);
    func(l, p, 0);
  }
}

pub use lua_pushlightuserdata as LUA_PUSHLIGHTUSERDATA;
