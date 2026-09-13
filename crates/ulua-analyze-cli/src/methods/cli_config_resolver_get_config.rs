use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_cli_lib::functions::get_parent_path::get_parent_path;
use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

use crate::records::cli_config_resolver::CliConfigResolver;

/// `const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CliConfigResolver`.
pub(crate) unsafe fn cli_config_resolver_get_config_thunk(
  this: *const ConfigResolver,
  name: *const ModuleName,
  limits: *const TypeCheckLimits,
) -> *const Config {
  unsafe {
    let this = this as *const CliConfigResolver;
    (*this).get_config(&*name, &*limits) as *const Config
  }
}

impl CliConfigResolver {
  /// C++ `const Config& getConfig(const ModuleName& name, const TypeCheckLimits& limits) const`
  /// (`CLI/src/Analyze.cpp:243-250`).
  ///
  /// `getConfig` is logically `const` in C++ but mutates the `mutable` `configCache`
  /// / `configErrors` members through `readConfigRec`; the `&self` receiver is cast
  /// to `&mut self` for those interior mutations, matching the C++ `mutable` intent.
  pub fn get_config(&self, name: &ModuleName, limits: &TypeCheckLimits) -> &Config {
    // std::optional<std::string> path = getParentPath(name);
    // if (!path) return defaultConfig;
    let path = match get_parent_path(name) {
      None => return &self.default_config,
      Some(path) => path,
    };

    // return readConfigRec(*path, limits);
    let this = self as *const CliConfigResolver as *mut CliConfigResolver;
    unsafe { (*this).read_config_rec(&path, limits) }
  }
}
