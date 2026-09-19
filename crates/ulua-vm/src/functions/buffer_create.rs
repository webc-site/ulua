use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, lua_newbuffer::lua_newbuffer},
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `lua_State`.
pub(crate) unsafe extern "C-unwind" fn buffer_create(l: *mut lua_State) -> c_int {
  unsafe {
    let size = lua_l_checkinteger(l, 1);

    luaL_argcheck!(l, size >= 0, 1, "size");

    lua_newbuffer(l, size as usize);
    1
  }
}
