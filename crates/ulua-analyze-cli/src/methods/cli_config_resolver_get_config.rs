use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_cli_lib::functions::get_parent_path::get_parent_path;
use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

use crate::records::cli_config_resolver::CliConfigResolver;

impl ConfigResolver for CliConfigResolver {
  /// C++ `const Config& getConfig(const ModuleName& name, const TypeCheckLimits& limits) const`
  /// (`CLI/src/Analyze.cpp:243-250`).
  ///
  /// 逻辑 const、经 `UnsafeCell`/`RefCell` 填充 `mutable` 的 configCache/configErrors
  /// （见 [`CliConfigResolver`] 字段文档的单线程契约）。
  fn get_config(&self, name: &ModuleName, limits: &TypeCheckLimits) -> &Config {
    // std::optional<std::string> path = getParentPath(name);
    // if (!path) return defaultConfig;
    let path = match get_parent_path(name) {
      None => return &self.default_config,
      Some(path) => path,
    };

    // return readConfigRec(*path, limits);
    self.read_config_rec(&path, limits)
  }
}
