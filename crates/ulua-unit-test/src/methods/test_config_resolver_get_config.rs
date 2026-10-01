//! Source: `tests/Fixture.cpp`

use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

use crate::records::test_config_resolver::TestConfigResolver;

impl TestConfigResolver {
  pub fn get_config(&self, name: &ModuleName) -> &Config {
    self.config_files.get(name).unwrap_or(&self.default_config)
  }
}
