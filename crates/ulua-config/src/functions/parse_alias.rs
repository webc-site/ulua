use alloc::{format, string::String};

use crate::{
  functions::is_valid_alias::is_valid_alias,
  records::{alias_options::AliasOptions, config::Config},
  type_aliases::error::Error,
};

pub(crate) fn parse_alias(
  config: &mut Config,
  alias_key: &str,
  alias_value: &str,
  alias_options: &Option<AliasOptions>,
) -> Error {
  if !is_valid_alias(alias_key) {
    return Some(format!("Invalid alias {alias_key}"));
  }

  let Some(options) = alias_options else {
    return Some(String::from("Cannot parse aliases without alias options"));
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

  None
}
