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
  convert_repl_requirer::luarequire_NavigateResult as CliNavigateResult,
  convert_repl_requirer_alt_b::luarequire_ConfigStatus as CliConfigStatus,
  get_cache_key::get_cache_key, get_chunkname::get_chunkname, get_config::get_config,
  get_config_status::get_config_status, get_loadname::get_loadname,
  is_module_present::is_module_present, is_require_allowed::is_require_allowed,
  jump_to_alias::jump_to_alias, load::load, reset::reset, to_child::to_child, to_parent::to_parent,
  write::luarequire_WriteResult as CliWriteResult,
};

// The CLI requirer functions report results with crate-local enums (the faithful
// translations of the enums declared alongside Repl.cpp/ReplRequirer.cpp); these
// helpers map them onto the `ulua_require` C-ABI enums expected by the
// luarequire_Configuration struct that luaopen_require consumes.
fn to_req_navigate(r: CliNavigateResult) -> ReqNavigateResult {
  match r {
    CliNavigateResult::NAVIGATE_SUCCESS => ReqNavigateResult::NAVIGATE_SUCCESS,
    CliNavigateResult::NAVIGATE_AMBIGUOUS => ReqNavigateResult::NAVIGATE_AMBIGUOUS,
    CliNavigateResult::NAVIGATE_NOT_FOUND => ReqNavigateResult::NAVIGATE_NOT_FOUND,
  }
}

fn to_req_config_status(r: CliConfigStatus) -> ReqConfigStatus {
  match r {
    CliConfigStatus::ConfigAmbiguous => ReqConfigStatus::ConfigAmbiguous,
    CliConfigStatus::ConfigPresentJson => ReqConfigStatus::ConfigPresentJson,
    CliConfigStatus::ConfigPresentLuau => ReqConfigStatus::ConfigPresentLuau,
    CliConfigStatus::ConfigAbsent => ReqConfigStatus::ConfigAbsent,
  }
}

fn to_req_write(r: CliWriteResult) -> ReqWriteResult {
  match r {
    CliWriteResult::WRITE_SUCCESS => ReqWriteResult::WRITE_SUCCESS,
    CliWriteResult::WRITE_BUFFER_TOO_SMALL => ReqWriteResult::WRITE_BUFFER_TOO_SMALL,
    CliWriteResult::WRITE_FAILURE => ReqWriteResult::WRITE_FAILURE,
  }
}

unsafe extern "C-unwind" fn cb_is_require_allowed(
  l: *mut c_void,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> bool {
  unsafe { is_require_allowed(l as *mut lua_State, ctx, requirer_chunkname) }
}

unsafe extern "C-unwind" fn cb_reset(
  l: *mut c_void,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> ReqNavigateResult {
  unsafe { to_req_navigate(reset(l as *mut lua_State, ctx, requirer_chunkname)) }
}

unsafe extern "C-unwind" fn cb_jump_to_alias(
  l: *mut c_void,
  ctx: *mut c_void,
  path: *const c_char,
) -> ReqNavigateResult {
  unsafe { to_req_navigate(jump_to_alias(l as *mut lua_State, ctx, path)) }
}

unsafe extern "C-unwind" fn cb_to_parent(l: *mut c_void, ctx: *mut c_void) -> ReqNavigateResult {
  unsafe { to_req_navigate(to_parent(l as *mut lua_State, ctx)) }
}

unsafe extern "C-unwind" fn cb_to_child(
  l: *mut c_void,
  ctx: *mut c_void,
  name: *const c_char,
) -> ReqNavigateResult {
  unsafe { to_req_navigate(to_child(l as *mut lua_State, ctx, name)) }
}

unsafe extern "C-unwind" fn cb_is_module_present(l: *mut c_void, ctx: *mut c_void) -> bool {
  unsafe { is_module_present(l as *mut lua_State, ctx) }
}

unsafe extern "C-unwind" fn cb_get_config_status(
  l: *mut c_void,
  ctx: *mut c_void,
) -> ReqConfigStatus {
  unsafe { to_req_config_status(get_config_status(l as *mut lua_State, ctx)) }
}

unsafe extern "C-unwind" fn cb_get_chunkname(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe {
    to_req_write(get_chunkname(
      l as *mut lua_State,
      ctx,
      buffer,
      buffer_size,
      size_out,
    ))
  }
}

unsafe extern "C-unwind" fn cb_get_loadname(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe {
    to_req_write(get_loadname(
      l as *mut lua_State,
      ctx,
      buffer,
      buffer_size,
      size_out,
    ))
  }
}

unsafe extern "C-unwind" fn cb_get_cache_key(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe {
    to_req_write(get_cache_key(
      l as *mut lua_State,
      ctx,
      buffer,
      buffer_size,
      size_out,
    ))
  }
}

unsafe extern "C-unwind" fn cb_get_config(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> ReqWriteResult {
  unsafe {
    to_req_write(get_config(
      l as *mut lua_State,
      ctx,
      buffer,
      buffer_size,
      size_out,
    ))
  }
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
