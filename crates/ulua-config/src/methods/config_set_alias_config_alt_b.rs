use alloc::string::String;

use crate::records::config::Config;

impl Config {
  /// 对应 C++ `Config::setAlias(alias, value, configLocation)`。
  ///
  /// 等价 `set_alias` + 回填 configLocation，但单次 `get_or_insert` 哈希查找
  /// （原先小写化与哈希各做两次）。
  pub fn set_alias_with_location(&mut self, alias: String, value: String, config_location: &str) {
    let info = self.aliases.get_or_insert(alias.to_ascii_lowercase());
    info.value = value;
    info.original_case = alias;
    info.config_location = config_location.to_string();
  }
}
