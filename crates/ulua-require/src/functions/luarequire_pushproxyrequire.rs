use core::ffi::c_void;

use ulua_vm::records::lua_state::lua_State;

use crate::{
  functions::{
    lua_proxyrequire::lua_proxyrequire, pushrequireclosureinternal::pushrequireclosureinternal,
  },
  records::luarequire_configuration::LuarequireConfigurationInit,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
pub unsafe fn luarequire_pushproxyrequire(
  l: *mut lua_State,
  config_init: LuarequireConfigurationInit,
  ctx: *mut c_void,
) -> i32 {
  unsafe {
    pushrequireclosureinternal(
      l,
      config_init,
      ctx,
      Some(lua_proxyrequire),
      c"proxyrequire".as_ptr(),
    )
  }
}
