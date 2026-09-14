use alloc::string::String;

use crate::records::config::Config;

impl Config {
  /// 对应 C++ `Config::setAlias(alias, value, configLocation)`。
  pub fn set_alias_with_location(&mut self, alias: String, value: String, config_location: &str) {
    let alias_lower = alias.to_ascii_lowercase();
    self.set_alias(alias, value);

    self.aliases.get_or_insert(alias_lower).config_location = config_location.to_string();
  }
}
