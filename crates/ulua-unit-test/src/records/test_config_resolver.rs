//! Source: `tests/Fixture.h`

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
