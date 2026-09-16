use alloc::string::String;

use crate::{
  functions::{
    create_luau_config_from_luau_table::parse_luau_config_table,
    extract_config::{execute_and_extract, new_sandbox},
    load::load_bytecode,
  },
  records::{alias_options::AliasOptions, config::Config, interrupt_callbacks::InterruptCallbacks},
};

/// 对应 C++ `extractLuauConfigFromBytecode`：自建沙箱 VM，加载预编译字节码并解析配置。
pub fn extract_luau_config_from_bytecode(
  bytecode: &[u8],
  config: &mut Config,
  alias_options: Option<AliasOptions>,
  callbacks: InterruptCallbacks,
) -> Option<String> {
  let state = new_sandbox();
  let l = state.0;

  if let Some(load_error) = load_bytecode(l, bytecode) {
    return Some(load_error);
  }

  let mut error = String::new();
  let Some(config_table) = execute_and_extract(l, &callbacks, &mut error) else {
    return Some(error);
  };

  parse_luau_config_table(&config_table, config, alias_options)
}
