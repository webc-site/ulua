use core::ffi::c_void;

use ulua_vm::records::lua_state::lua_State;

use crate::{
  functions::{lua_require::lua_require, pushrequireclosureinternal::pushrequireclosureinternal},
  records::luarequire_configuration::LuarequireConfigurationInit,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe fn luarequire_pushrequire(
  l: *mut lua_State,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
) -> i32 {
  unsafe { pushrequireclosureinternal(l, config_init, ctx, Some(lua_require), c"require".as_ptr()) }
}
