//! Node: `cxx:Record:Luau.Analyze.CLI:CLI/src/Analyze.cpp:231:cli_config_resolver`
//! Source: `CLI/src/Analyze.cpp`
//! Graph edges:
//! - declared_by: source_file CLI/src/Analyze.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Config/include/Luau/LuauConfig.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Ast/include/Luau/PrettyPrinter.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file Common/include/Luau/TimeTrace.h
//!   - includes -> source_file Analysis/include/Luau/TypeAttach.h
//!   - includes -> source_file Analysis/include/Luau/TypeCheckLimits.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file CLI/include/Luau/AnalyzeRequirer.h
//!   - includes -> source_file CLI/include/Luau/FileUtils.h
//!   - includes -> source_file CLI/include/Luau/Flags.h
//!   - includes -> source_file Require/include/Luau/RequireNavigator.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//! - incoming:
//!   - declares <- source_file CLI/src/Analyze.cpp
//!   - type_ref <- function main (CLI/src/Analyze.cpp)
//!   - type_ref <- method CliConfigResolver::CliConfigResolver (CLI/src/Analyze.cpp)
//!   - type_ref <- method CliConfigResolver::getConfig (CLI/src/Analyze.cpp)
//!   - type_ref <- method CliConfigResolver::readConfigRec (CLI/src/Analyze.cpp)
//! - outgoing:
//!   - type_ref -> method CliConfigResolver::CliConfigResolver (CLI/src/Analyze.cpp)
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - translates_to -> rust_item CliConfigResolver

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
