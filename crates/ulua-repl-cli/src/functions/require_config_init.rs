use core::ffi::{c_char, c_int, c_void};

use ulua_require::{
  enums::{
    luarequire_config_status::luarequire_ConfigStatus as ReqConfigStatus,
    luarequire_navigate_result::luarequire_NavigateResult as ReqNavigateResult,
    luarequire_write_result::luarequire_WriteResult as ReqWriteResult,
  },
  records::luarequire_configuration::luarequire_Configuration,
};
use ulua_vm::type_aliases::lua_state::lua_State;

use crate::functions::{
  get_cache_key::get_cache_key, get_chunkname::get_chunkname, get_config::get_config,
  get_config_status::get_config_status, get_loadname::get_loadname,
  is_module_present::is_module_present, is_require_allowed::is_require_allowed,
  jump_to_alias::jump_to_alias, load::load, reset::reset, to_child::to_child, to_parent::to_parent,
};

// CLI 回调直接返回 `ulua_require` 的 C-ABI 枚举 (luaopen_require 消费的
// luarequire_Configuration 所要求的类型); 旧版 CLI 局部枚举到 Req 枚举的
// 映射在两侧类型统一后已成恒等映射, 不再保留。

unsafe extern "C-unwind" fn cb_is_require_allowed(
  l: *mut c_void,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> bool {
  unsafe { is_require_allowed(l, ctx, requirer_chunkname) }
}

unsafe extern "C-unwind" fn cb_reset(
  l: *mut c_void,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> ReqNavigateResult {
  unsafe { reset(l as *mut lua_State, ctx, requirer_chunkname) }
}

unsafe extern "C-unwind" fn cb_jump_to_alias(
  l: *mut c_void,
  ctx: *mut c_void,
  path: *const c_char,
) -> ReqNavigateResult {
  unsafe { jump_to_alias(l as *mut lua_State, ctx, path) }
}

unsafe extern "C-unwind" fn cb_to_parent(l: *mut c_void, ctx: *mut c_void) -> ReqNavigateResult {
  unsafe { to_parent(l as *mut lua_State, ctx) }
}

unsafe extern "C-unwind" fn cb_to_child(
  l: *mut c_void,
  ctx: *mut c_void,
  name: *const c_char,
) -> ReqNavigateResult {
  unsafe { to_child(l as *mut lua_State, ctx, name) }
}

unsafe extern "C-unwind" fn cb_is_module_present(l: *mut c_void, ctx: *mut c_void) -> bool {
  unsafe { is_module_present(l as *mut lua_State, ctx) }
}

unsafe extern "C-unwind" fn cb_get_config_status(
  l: *mut c_void,
  ctx: *mut c_void,
) -> ReqConfigStatus {
  unsafe { get_config_status(l as *mut lua_State, ctx) }
}

unsafe extern "C-unwind" fn cb_get_chunkname(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe { get_chunkname(l as *mut lua_State, ctx, buffer, buffer_size, size_out) }
}

unsafe extern "C-unwind" fn cb_get_loadname(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe { get_loadname(l as *mut lua_State, ctx, buffer, buffer_size, size_out) }
}

unsafe extern "C-unwind" fn cb_get_cache_key(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe { get_cache_key(l as *mut lua_State, ctx, buffer, buffer_size, size_out) }
}

unsafe extern "C-unwind" fn cb_get_config(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe { get_config(l as *mut lua_State, ctx, buffer, buffer_size, size_out) }
}

unsafe extern "C-unwind" fn cb_load(
  l: *mut c_void,
  ctx: *mut c_void,
  path: *const c_char,
  chunkname: *const c_char,
  loadname: *const c_char,
) -> c_int {
  unsafe { load(l as *mut lua_State, ctx, path, chunkname, loadname) }
}

/// # Safety
///
/// If `config` is non-null, it must be properly aligned and point to a valid `luarequire_Configuration`.
pub unsafe extern "C-unwind" fn require_config_init(config: *mut luarequire_Configuration) {
  unsafe {
    if config.is_null() {
      return;
    }

    let config = &mut *config;
    config.is_require_allowed = Some(cb_is_require_allowed);
    config.reset = Some(cb_reset);
    config.jump_to_alias = Some(cb_jump_to_alias);
    config.to_parent = Some(cb_to_parent);
    config.to_child = Some(cb_to_child);
    config.is_module_present = Some(cb_is_module_present);
    config.get_config_status = Some(cb_get_config_status);
    config.get_chunkname = Some(cb_get_chunkname);
    config.get_loadname = Some(cb_get_loadname);
    config.get_cache_key = Some(cb_get_cache_key);
    config.get_config = Some(cb_get_config);
    config.load = Some(cb_load);
  }
}
