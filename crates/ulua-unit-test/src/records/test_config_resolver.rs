//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Fixture.h:108:test_config_resolver`
//! Source: `tests/Fixture.h`
//! Graph edges:
//! - declared_by: source_file tests/Fixture.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinTypeFunctions.h
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Ast/include/Luau/Location.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/IostreamOptional.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Fixture.h
//!   - type_ref <- record Fixture (tests/Fixture.h)
//!   - type_ref <- test frontend_check_without_builtin_next (tests/Frontend.test.cpp)
//!   - type_ref <- method TestConfigResolver::getConfig (tests/Fixture.cpp)
//! - outgoing:
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - translates_to -> rust_item TestConfigResolver

use std::collections::HashMap;

use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

#[derive(Debug)]
#[repr(C)]
pub struct TestConfigResolver {
  pub base: ConfigResolver,
  pub default_config: Config,
  pub config_files: HashMap<ModuleName, Config>,
}

impl Default for TestConfigResolver {
  fn default() -> Self {
    Self {
      base: ConfigResolver {
        get_config: Some(test_config_resolver_get_config),
      },
      default_config: Config::default(),
      config_files: HashMap::new(),
    }
  }
}

unsafe fn test_config_resolver_get_config(
  this: *const ConfigResolver,
  name: *const ModuleName,
  _limits: *const TypeCheckLimits,
) -> *const Config {
  let resolver = this as *const TestConfigResolver;
  let name = unsafe { &*name };
  unsafe { (*resolver).get_config(name) as *const Config }
}
