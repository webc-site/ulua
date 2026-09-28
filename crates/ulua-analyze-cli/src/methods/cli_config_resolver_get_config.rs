use alloc::string::String;
use core::cell::Ref;

use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_cli_lib::functions::get_parent_path::get_parent_path;
use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

use crate::records::cli_config_resolver::CliConfigResolver;

/// `const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const` thunk.
///
/// # Safety
/// `this` 须指向存活 `CliConfigResolver` 的 `base` 子对象（`#[repr(C)]` 首字段，
/// 基址与 `ConfigResolver*` 重合）；`name`/`limits` 须为非空且指向存活对象的合法指针。
pub(crate) unsafe fn cli_config_resolver_get_config_thunk(
  this: *const ConfigResolver,
  name: *const ModuleName,
  limits: *const TypeCheckLimits,
) -> *const Config {
  // Safety: 调用方满足函数级 # Safety 契约，`this` 指向存活 CliConfigResolver 的
  // base 子对象（#[repr(C)] 首字段，基址重合，向上转 *const ConfigResolver→
  // *const CliConfigResolver 合法）；`name`/`limits` 按契约非空且指向存活对象，
  // 本帧只读解引用后立即收敛，不写穿；`get_config` 返回的 &Config 由
  // CliConfigResolver 的 Box 堆缓存保地址稳定，*const Config 在调用方读取窗口内
  // 持续有效。
  unsafe {
    let this = this as *const CliConfigResolver;
    (*this).get_config(&*name, &*limits) as *const Config
  }
}

impl CliConfigResolver {
  /// C++ `const Config& getConfig(const ModuleName& name, const TypeCheckLimits& limits) const`
  /// (`CLI/src/Analyze.cpp:243-250`).
  ///
  /// 逻辑 const、经 `UnsafeCell`/`RefCell` 填充 `mutable` 的 configCache/configErrors
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
  pub(crate) fn config_errors(&self) -> Ref<'_, [(String, String)]> {
    Ref::map(self.config_errors.borrow(), |v| v.as_slice())
  }
}
