//! Port of `DemoConfigResolver : Luau::ConfigResolver` (`CLI/src/Web.cpp:49-62`).
//!
//! The luau.org/demo config resolver. Like the analysis `ConfigResolver` (a struct
//! with a fn-pointer vtable slot for the single pure virtual `getConfig`), this
//! concrete subclass is `#[repr(C)]` with `base: ConfigResolver` first so a
//! `*const ConfigResolver` (the vtable receiver) can be cast back to
//! `*const DemoConfigResolver` to reach `default_config`.
//!
//! C++ member: `Luau::Config defaultConfig;`.

use ulua_analysis::{
  records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::enums::mode::Mode;
use ulua_config::records::config::Config;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct DemoConfigResolver {
  pub base: ConfigResolver,
  pub default_config: Config,
}

/// cpp `DemoConfigResolver()` 构造段（`CLI/src/Web.cpp:51-54`）硬置
/// `Mode::Strict`：
///
/// ```cpp
/// DemoConfigResolver() { defaultConfig.mode = Luau::Mode::Strict; }
/// ```
///
/// DELIBERATE DEVIATION from `Web.cpp`：playground 默认 `Nonstrict`，让每段脚本
/// 自己的 `--!strict` / `--!nonstrict` 模式注释说了算（与 `ulua-analyze` CLI 同向）。
/// 如此才不至于把未标注的示例脚本按 strict 检查、向访问者报出「干净代码被判有错」
/// 的困惑诊断。
impl Default for DemoConfigResolver {
  fn default() -> Self {
    let default_config = Config {
      mode: Mode::Nonstrict,
      ..Default::default()
    };

    DemoConfigResolver {
      base: ConfigResolver {
        get_config: Some(demo_config_resolver_get_config_thunk),
      },
      default_config,
    }
  }
}

impl DemoConfigResolver {
  /// `const Luau::Config& getConfig(const ModuleName&, const TypeCheckLimits&) const`
  /// (`CLI/src/Web.cpp:56-59`)：恒返回同一份默认配置，忽略模块名与预算。
  fn get_config(&self, _name: &ModuleName, _limits: &TypeCheckLimits) -> &Config {
    &self.default_config
  }
}

/// `getConfig` 的 vtable thunk。
///
/// # Safety
/// `this` must point at the `base` subobject of a live `DemoConfigResolver`.
pub(crate) unsafe fn demo_config_resolver_get_config_thunk(
  this: *const ConfigResolver,
  name: *const ModuleName,
  limits: *const TypeCheckLimits,
) -> *const Config {
  unsafe {
    let this = this as *const DemoConfigResolver;
    (*this).get_config(&*name, &*limits) as *const Config
  }
}
