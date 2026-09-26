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

  // DenseHashMap<String, _> 查询需要 &String
  let alias_key = String::from(alias_key);

  if options.overwrite_aliases || !config.aliases.contains(&alias_key) {
    if let Some(config_location) = &options.config_location {
      config.set_alias_with_location(alias_key, String::from(alias_value), config_location);
    } else {
      config.set_alias(alias_key, String::from(alias_value));
    }
  }

  Ok(())
}
