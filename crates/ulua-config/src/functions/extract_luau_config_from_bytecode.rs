use crate::{
  error::ConfigError,
  functions::{
    create_luau_config_from_luau_table::parse_luau_config_table, extract_config::run_in_sandbox,
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
) -> Result<(), ConfigError> {
  let config_table = run_in_sandbox(|l| load_bytecode(l, bytecode), &callbacks)?;
  parse_luau_config_table(&config_table, config, alias_options)
}
