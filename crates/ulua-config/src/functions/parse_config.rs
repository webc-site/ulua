use alloc::format;

use crate::{
  error::ConfigError,
  functions::{
    parse_alias::parse_alias, parse_boolean::parse_boolean, parse_json::parse_json,
    parse_lint_rule_string::parse_lint_rule_string, parse_mode_string::parse_mode_string,
  },
  records::{config::Config, config_options::ConfigOptions},
};

/// 对应 C++ `parseConfig`：解析 JSON 配置文件并套用到 `config`，
/// 错误统一经 [`ConfigError`] 传播（`Err` 对应 cpp 的 `Error`）。
pub fn parse_config(
  contents: &str,
  config: &mut Config,
  options: &ConfigOptions,
) -> Result<(), ConfigError> {
  parse_json(contents, &mut |keys, value| -> Result<(), ConfigError> {
    match keys {
      [key] if key == "languageMode" => config.mode = parse_mode_string(&value, options.compat)?,
      [key0, key1] if key0 == "lint" => parse_lint_rule_string(
        &mut config.enabled_lint,
        &mut config.fatal_lint,
        key1,
        &value,
        options.compat,
      )?,
      [key] if key == "lintErrors" => config.lint_errors = parse_boolean(&value)?,
      [key] if key == "typeErrors" => config.type_errors = parse_boolean(&value)?,
      [key] if key == "globals" => config.globals.push(value),
      [key0, key1] if key0 == "aliases" => {
        parse_alias(config, key1, &value, options.alias_options.as_ref())?;
      }
      // compat 模式兼容旧键 "language"."mode"
      [key0, key1] if options.compat && key0 == "language" && key1 == "mode" => {
        config.mode = parse_mode_string(&value, options.compat)?;
      }
      _ => {
        return Err(ConfigError::Message(format!(
          "Unknown key {}",
          keys.join("/")
        )));
      }
    }
    Ok(())
  })
}
