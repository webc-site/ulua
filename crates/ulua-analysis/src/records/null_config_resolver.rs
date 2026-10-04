use ulua_config::records::config::Config;

use crate::{
  records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};

/// C++ `struct NullConfigResolver : ConfigResolver`
/// （`Analysis/include/Luau/ConfigResolver.h:19`）：对所有模块恒返回同一份默认
/// 配置的空 resolver。`Frontend` 以 `Box<dyn ConfigResolver>` 独占持有配置解析器
/// 后，cpp 侧「传 nullptr、从不查询 getConfig」的缺位场景改由本活实例表达
/// （对齐 `NullFileResolver` 的收口手法）。
#[derive(Debug, Default)]
pub struct NullConfigResolver {
  default_config: Config,
}

impl NullConfigResolver {
  pub fn new() -> Self {
    Self::default()
  }
}

impl ConfigResolver for NullConfigResolver {
  fn get_config(&self, _name: &ModuleName, _limits: &TypeCheckLimits) -> &Config {
    &self.default_config
  }
}
