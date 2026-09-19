//! `DemoFileResolver::getEnvironmentForModule` (`CLI/src/Web.cpp:40-43`).
//!
//! ```cpp
//! std::optional<std::string> getEnvironmentForModule(const Luau::ModuleName& name) const override
//! {
//!     return std::nullopt;
//! }
//! ```

use alloc::string::String;

use ulua_analysis::type_aliases::module_name_type::ModuleName;

use crate::records::demo_file_resolver::DemoFileResolver;

impl DemoFileResolver {
  pub fn get_environment_for_module(&self, _name: &ModuleName) -> Option<String> {
    None
  }
}
