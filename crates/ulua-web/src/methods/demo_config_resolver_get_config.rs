//! `DemoConfigResolver::getConfig` (`CLI/src/Web.cpp:56-59`).
//!
//! ```cpp
//! virtual const Luau::Config& getConfig(const Luau::ModuleName& name, const Luau::TypeCheckLimits& limits) const override
//! {
//!     return defaultConfig;
//! }
//! ```

use ulua_analysis::{
  records::type_check_limits::TypeCheckLimits, type_aliases::module_name_type::ModuleName,
};
use ulua_config::records::config::Config;

use crate::records::demo_config_resolver::DemoConfigResolver;

impl DemoConfigResolver {
  pub fn get_config(&self, _name: &ModuleName, _limits: &TypeCheckLimits) -> &Config {
    &self.default_config
  }
}
