use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::lua_State};

use crate::records::luarequire_configuration::luarequire_Configuration;

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `lua_State`.
pub unsafe fn validate_config(l: *mut lua_State, config: &luarequire_Configuration) {
  unsafe {
    // 必填指针清单（(是否存在, 名称)），校验顺序对应 cpp；首个缺失项报错中止
    let required = [
      (config.is_require_allowed.is_some(), "is_require_allowed"),
      (config.reset.is_some(), "reset"),
      (config.jump_to_alias.is_some(), "jump_to_alias"),
      (config.to_parent.is_some(), "to_parent"),
      (config.to_child.is_some(), "to_child"),
      (config.is_module_present.is_some(), "is_module_present"),
      (config.get_chunkname.is_some(), "get_chunkname"),
      (config.get_loadname.is_some(), "get_loadname"),
      (config.get_cache_key.is_some(), "get_cache_key"),
      (config.get_config_status.is_some(), "get_config_status"),
    ];
    for (present, name) in required {
      if !present {
        luaL_error!(
          l,
          "require configuration is missing required function pointer: {name}"
        );
      }
    }

    // get_alias 与 get_config 二选一
    if config.get_alias.is_some() && config.get_config.is_some() {
      luaL_error!(
        l,
        "require configuration cannot define both get_alias and get_config"
      );
    }
    if config.get_alias.is_none() && config.get_config.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: either get_alias or get_config (not both)"
      );
    }
    if config.load.is_none() {
      luaL_error!(
        l,
        "require configuration is missing required function pointer: load"
      );
    }
  }
}
