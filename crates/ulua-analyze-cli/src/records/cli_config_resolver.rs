//! Port of `CliConfigResolver`（`CLI/src/Analyze.cpp:231-321`）。

use alloc::{string::String, vec::Vec};
use std::collections::HashMap;

use ulua_analysis::records::config_resolver::ConfigResolver;
use ulua_config::records::config::Config;

/// Port of `struct CliConfigResolver : Luau::ConfigResolver` (`CLI/src/Analyze.cpp:231-321`).
///
/// Like the analysis `ConfigResolver` (a struct with a fn-pointer vtable slot for
/// the single pure virtual `getConfig`), this concrete subclass is `#[repr(C)]`
/// with `base: ConfigResolver` first so that a `*const ConfigResolver` (the vtable
/// receiver) can be cast back to `*const CliConfigResolver`.
///
/// C++ members:
/// - `Luau::Config defaultConfig;`
/// - `mutable std::unordered_map<std::string, Luau::Config> configCache;`
/// - `mutable std::vector<std::pair<std::string, std::string>> configErrors;`
#[repr(C)]
#[derive(Debug)]
pub struct CliConfigResolver {
  pub base: ConfigResolver,
  pub default_config: Config,
  pub config_cache: HashMap<String, Config>,
  pub config_errors: Vec<(String, String)>,
}
