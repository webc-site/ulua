//! `DemoFileResolver::getHumanReadableModuleName` (`CLI/src/Web.cpp:35-38`).
//!
//! ```cpp
//! std::string getHumanReadableModuleName(const Luau::ModuleName& name) const override
//! {
//!     return name;
//! }
//! ```

use alloc::string::String;

use ulua_analysis::type_aliases::module_name_type::ModuleName;

use crate::records::demo_file_resolver::DemoFileResolver;

impl DemoFileResolver {
  pub fn get_human_readable_module_name(&self, name: &ModuleName) -> String {
    name.clone()
  }
}
