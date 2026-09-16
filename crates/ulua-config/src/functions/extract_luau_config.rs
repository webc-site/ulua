use alloc::string::String;

use crate::{
  functions::{
    create_luau_config_from_luau_table::parse_luau_config_table, extract_config::extract_config,
  },
  records::{alias_options::AliasOptions, config::Config, interrupt_callbacks::InterruptCallbacks},
};

/// 对应 C++ `extractLuauConfig`：执行 `source` 并把 "luau" 表套用到 `config`。
pub fn extract_luau_config(
  source: &str,
  config: &mut Config,
  alias_options: Option<AliasOptions>,
  callbacks: InterruptCallbacks,
) -> Option<String> {
  let mut error = String::new();
  let Some(config_table) = extract_config(source, &callbacks, &mut error) else {
    return Some(error);
  };

  parse_luau_config_table(&config_table, config, alias_options)
}
