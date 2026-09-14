use alloc::string::{String, ToString};

use ulua_analysis::type_aliases::module_name_type::ModuleName;

/// C++ `std::string CliFileResolver::getHumanReadableModuleName(const ModuleName& name) const`
/// (`CLI/src/Analyze.cpp:216-220`)。
pub fn cli_file_resolver_get_human_readable_module_name(name: &ModuleName) -> String {
  if name == "-" {
    "stdin".to_string()
  } else {
    name.clone()
  }
}
