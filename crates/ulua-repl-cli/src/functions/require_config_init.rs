//! cpp `ReplRequirer.cpp` 末尾的 `requireConfigInit(luarequire_Configuration*)`。
//!
//! 各回调本身就是 `luarequire_Configuration` 要求的 C-ABI 函数指针
//! （`unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, ...)`），故直接赋值，
//! 无需额外的适配层。

use ulua_cli_lib::functions::is_require_allowed::is_require_allowed;
use ulua_require::records::luarequire_configuration::luarequire_Configuration;

use crate::functions::{
  get_cache_key::get_cache_key, get_chunkname::get_chunkname, get_config::get_config,
  get_config_status::get_config_status, get_loadname::get_loadname,
  is_module_present::is_module_present, jump_to_alias::jump_to_alias, load::load, reset::reset,
  to_child::to_child, to_parent::to_parent,
};

/// # Safety
///
/// `config` 非空时必须对齐并指向有效的 `luarequire_Configuration`。
pub unsafe extern "C-unwind" fn require_config_init(config: *mut luarequire_Configuration) {
  if config.is_null() {
    return;
  }

  let config = unsafe { &mut *config };
  config.is_require_allowed = Some(is_require_allowed);
  config.reset = Some(reset);
  config.jump_to_alias = Some(jump_to_alias);
  config.to_parent = Some(to_parent);
  config.to_child = Some(to_child);
  config.is_module_present = Some(is_module_present);
  config.get_config_status = Some(get_config_status);
  config.get_chunkname = Some(get_chunkname);
  config.get_loadname = Some(get_loadname);
  config.get_cache_key = Some(get_cache_key);
  config.get_config = Some(get_config);
  config.load = Some(load);
}
