use core::ffi::{c_char, c_int};

use crate::{
  functions::lua_pushcclosurek::lua_pushcclosurek,
  type_aliases::{lua_c_function::LuaCFunction, lua_state::lua_State},
};

/// # Safety
///
/// `l` must point to a valid, live `lua_State`.
#[inline(always)]
pub unsafe fn lua_pushcclosure(
  l: *mut lua_State,
  f: LuaCFunction,
  debugname: *const c_char,
  nup: c_int,
) {
  unsafe {
    lua_pushcclosurek(l, f, debugname, nup, None);
  }
}

pub use lua_pushcclosure as LUA_PUSHCCLOSURE;
