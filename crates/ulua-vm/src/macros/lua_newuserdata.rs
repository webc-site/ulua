use core::ffi::c_void;

use crate::{
  functions::lua_newuserdatatagged::lua_newuserdatatagged, records::lua_state::lua_State,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `lua_State`.
#[inline]
pub unsafe fn lua_newuserdata(l: *mut lua_State, s: usize) -> *mut c_void {
  unsafe { lua_newuserdatatagged(l, s, 0) }
}
