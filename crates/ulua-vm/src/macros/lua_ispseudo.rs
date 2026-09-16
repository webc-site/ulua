use core::ffi::c_int;

use crate::macros::lua_registryindex::LUA_REGISTRYINDEX;
#[inline(always)]
pub const fn lua_ispseudo(i: c_int) -> bool {
  i <= LUA_REGISTRYINDEX
}
