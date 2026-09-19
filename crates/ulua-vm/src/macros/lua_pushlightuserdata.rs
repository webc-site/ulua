use core::ffi::c_void;

use crate::{
  functions::lua_pushlightuserdatatagged::lua_pushlightuserdatatagged,
  type_aliases::lua_state::lua_State,
};

/// cpp `lua.h:519` `#define lua_pushlightuserdata(L, p) lua_pushlightuserdatatagged(L, p, 0)` 对应。
///
/// # Safety
///
/// `l` 必须指向存活的 `lua_State`。
#[inline(always)]
pub unsafe fn lua_pushlightuserdata(l: *mut lua_State, p: *mut c_void) {
  unsafe { lua_pushlightuserdatatagged(l, p, 0) };
}
