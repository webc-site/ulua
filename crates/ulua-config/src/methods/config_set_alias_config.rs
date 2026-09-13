use alloc::string::String;

use crate::records::config::Config;

impl Config {
  /// 对应 C++ `Config::setAlias(alias, value)`：键统一小写，清除 configLocation。
  pub fn set_alias(&mut self, alias: String, value: String) {
    let info = self.aliases.get_or_insert(alias.to_ascii_lowercase());
    info.value = value;
    info.original_case = alias;
    info.config_location = String::new();
  }
}
