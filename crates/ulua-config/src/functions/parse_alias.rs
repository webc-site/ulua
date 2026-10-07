use alloc::{format, string::String};

use crate::{
  error::ConfigError,
  functions::is_valid_alias::is_valid_alias,
  records::{alias_options::AliasOptions, config::Config},
};

pub(crate) fn parse_alias(
  config: &mut Config,
  alias_key: &str,
  alias_value: &str,
  alias_options: Option<&AliasOptions>,
) -> Result<(), ConfigError> {
  if !is_valid_alias(alias_key) {
    return Err(ConfigError::Message(format!("Invalid alias {alias_key}")));
  }

  let Some(options) = alias_options else {
    return Err(ConfigError::MissingAliasOptions);
  };

  // r7-rc-4 起 aliases 查询走 &str 借用口；键物化只发生在真正插入时（原
  // 「DenseHashMap<String, _> 查询需要 &String」的提前 String::from hack 删除）。
  if options.overwrite_aliases || !config.aliases.contains_str(alias_key) {
    let alias_key = String::from(alias_key);
    if let Some(config_location) = &options.config_location {
      config.set_alias_with_location(alias_key, String::from(alias_value), config_location);
    } else {
      config.set_alias(alias_key, String::from(alias_value));
    }
  }

  Ok(())
}
