//! Source: `tests/Fixture.h`

use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_common::collections::HashMap;
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
      config_files: HashMap::default(),
    }
  }
}

/// `ConfigResolver.get_config` 函数指针槽的 C ABI 回桥（cpp `getImpl` 同形）。
///
/// # Safety
/// `this` 是注册于包装器的 [`TestConfigResolver`] 非空地址（repr(C) 基类字段
/// 布局即 upcast，比本调用长寿）；`name` 指向调用方传入的存活 `ModuleName`
/// （本函数只读）；`_limits` 不触碰。返回 `Config` 指针由 resolver 容器保活至
/// 调用方使用期（cpp 返回 `const Config*` 同款所有权）。
unsafe fn test_config_resolver_get_config(
  this: *const ConfigResolver,
  name: *const ModuleName,
  _limits: *const TypeCheckLimits,
) -> *const Config {
  let resolver = this as *const TestConfigResolver;
  // Safety: 函数级 `# Safety` 契约即合法性证明：两处 `&*` 均物化帧内只读借用，
  // 判空与存活由契约保证。
  let (resolver, name) = unsafe { (&*resolver, &*name) };
  resolver.get_config(name) as *const Config
}
