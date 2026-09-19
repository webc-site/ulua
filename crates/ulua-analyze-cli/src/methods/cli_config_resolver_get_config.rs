use alloc::string::String;

use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_cli_lib::functions::get_parent_path::get_parent_path;
use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

use crate::records::cli_config_resolver::CliConfigResolver;

/// `const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CliConfigResolver`.
pub(crate) unsafe fn cli_config_resolver_get_config_thunk(
  this: *const ConfigResolver,
  name: *const ModuleName,
  limits: *const TypeCheckLimits,
) -> *const Config {
  unsafe {
    let this = this as *const CliConfigResolver;
    (*this).get_config(&*name, &*limits) as *const Config
  }
}

impl CliConfigResolver {
  /// C++ `const Config& getConfig(const ModuleName& name, const TypeCheckLimits& limits) const`
  /// (`CLI/src/Analyze.cpp:243-250`).
  ///
  /// 逻辑 const、经 `UnsafeCell` 填充 `mutable` 的 configCache/configErrors
  /// （见 [`CliConfigResolver`] 字段文档的单线程契约）。
  pub fn get_config(&self, name: &ModuleName, limits: &TypeCheckLimits) -> &Config {
    // std::optional<std::string> path = getParentPath(name);
    // if (!path) return defaultConfig;
    let path = match get_parent_path(name) {
      None => return &self.default_config,
      Some(path) => path,
    };

    // return readConfigRec(*path, limits);
    self.read_config_rec(&path, limits)
  }

  /// C++ `const std::vector<std::pair<std::string, std::string>>& getConfigErrors() const`。
  ///
  /// # Safety（类型级契约）
  /// resolver 单线程使用；读取时不得有其它借用正通过回调写这两个 cell。
  /// CLI 在全部模块检查结束后调用，满足契约。
  pub fn config_errors(&self) -> &[(String, String)] {
    // SAFETY: 见方法文档；此处无重叠可变借用。
    unsafe { &*self.config_errors.get() }
  }
}
