use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::lua_State};

use crate::records::luarequire_configuration::luarequire_Configuration;

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `lua_State`.
pub unsafe fn validate_config(l: *mut lua_State, config: &luarequire_Configuration) {
  unsafe {
    if config.is_require_allowed.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: is_require_allowed"
      );
      return;
    }
    if config.reset.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: reset"
      );
      return;
    }
    if config.jump_to_alias.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: jump_to_alias"
      );
      return;
    }
    if config.to_parent.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: to_parent"
      );
      return;
    }
    if config.to_child.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: to_child"
      );
      return;
    }
    if config.is_module_present.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: is_module_present"
      );
      return;
    }
    if config.get_chunkname.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: get_chunkname"
      );
      return;
    }
    if config.get_loadname.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: get_loadname"
      );
      return;
    }
    if config.get_cache_key.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: get_cache_key"
      );
      return;
    }
    if config.get_config_status.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: get_config_status"
      );
      return;
    }
    if config.get_alias.is_some() && config.get_config.is_some() {
      luaL_error!(
        l,
        "require configuration cannot define both get_alias and get_config"
      );
      return;
    }
    if config.get_alias.is_none() && config.get_config.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: either get_alias or get_config (not both)"
      );
      return;
    }
    if config.load.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: load"
      );
    }
  }
}
