use ulua_require::records::luarequire_configuration::luarequire_Configuration;

use crate::functions::{
  get_cache_key::get_cache_key, get_chunkname::get_chunkname, get_config::get_config,
  get_config_status::get_config_status, get_loadname::get_loadname,
  is_module_present::is_module_present, is_require_allowed::is_require_allowed,
  jump_to_alias::jump_to_alias, load::load, reset::reset, to_child::to_child, to_parent::to_parent,
};

/// Initializes the Luau require configuration.
///
/// # Safety
///
/// `config` must be either null or a valid pointer to a `luarequire_Configuration`.
pub unsafe extern "C-unwind" fn require_config_init(config: *mut luarequire_Configuration) {
  unsafe {
    if config.is_null() {
      return;
    }

    (*config).is_require_allowed = Some(is_require_allowed);
    (*config).reset = Some(reset);
    (*config).jump_to_alias = Some(jump_to_alias);
    (*config).to_alias_override = None;
    (*config).to_alias_fallback = None;
    (*config).to_parent = Some(to_parent);
    (*config).to_child = Some(to_child);
    (*config).is_module_present = Some(is_module_present);
    (*config).get_chunkname = Some(get_chunkname);
    (*config).get_loadname = Some(get_loadname);
    (*config).get_cache_key = Some(get_cache_key);
    (*config).get_config_status = Some(get_config_status);
    (*config).get_alias = None;
    (*config).get_config = Some(get_config);
    (*config).get_luau_config_timeout = None;
    (*config).load = Some(load);
  }
}
