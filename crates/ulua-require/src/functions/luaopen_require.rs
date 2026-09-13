use core::ffi::c_void;

use ulua_vm::{macros::lua_setglobal::lua_setglobal, records::lua_state::lua_State};

use crate::{
  functions::luarequire_pushrequire::luarequire_pushrequire,
  records::luarequire_configuration::LuarequireConfigurationInit,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe fn luaopen_require(
  l: *mut lua_State,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
) {
  unsafe {
    luarequire_pushrequire(l, config_init, ctx);
    lua_setglobal(l, c"require".as_ptr());
  }
}
