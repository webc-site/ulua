// 「取指定类型值，缺失即报 `BadValue(key, expected)`」七处同构样板的单源收口
// 于下方 [`typed`]：惰性构造错误，与原 `ok_or_else` 展开逐字等价。
use alloc::string::String;

use crate::{
  error::{ConfigError, VAL_BOOL, VAL_STR_ARRAY, VAL_STRING, VAL_TABLE},
  functions::{
    bad_setting::{SETTING_FALSE, SETTING_TRUE},
    parse_alias::parse_alias,
    parse_lint_rule_string::parse_lint_rule_string,
    parse_mode_string::parse_mode_string,
  },
  records::{
    alias_options::AliasOptions, config::Config, config_table::ConfigTable,
    config_table_key::ConfigTableKey, config_value::ConfigValue,
  },
};

/// 类型化取值门面：`None`（类型不符）即报 cpp
/// `configuration value for key "K" must be T`（[`ConfigError::bad_value`]）。
fn typed<T>(got: Option<T>, key: &str, expected: &'static str) -> Result<T, ConfigError> {
  got.ok_or_else(|| ConfigError::bad_value(key, expected))
}

/// 对应 C++ `createLuauConfigFromLuauTable`：把 "luau" 子表套用到 `config`。
pub(crate) fn create_luau_config_from_luau_table(
  config: &mut Config,
  luau_table: &ConfigTable,
  alias_options: Option<AliasOptions>,
) -> Result<(), ConfigError> {
  for (k, v) in luau_table.iter() {
    let ConfigTableKey::String(key) = k else {
      return Err(ConfigError::LuauTableKeyNotString);
    };

    match key.as_str() {
      "languagemode" => {
        let mode = typed(v.get_string(), "languagemode", VAL_STRING)?;

        config.mode = parse_mode_string(mode, false)?;
      }

      "lint" => {
        let lint = typed(v.get_table(), "lint", VAL_TABLE)?;

        // 先处理通配符，保证覆盖顺序符合预期
        if let Some(value) = lint.find_str("*") {
          apply_lint(config, "*", value)?;
        }

        for (k, v) in lint.iter() {
          let ConfigTableKey::String(warning_name) = k else {
            return Err(ConfigError::LintTableKeyNotString);
          };

          if warning_name == "*" {
            continue; // 上面已处理
          }

          apply_lint(config, warning_name, v)?;
        }
      }

      "linterrors" => {
        let value = typed(v.get_bool(), "linterrors", VAL_BOOL)?;
        config.lint_errors = *value;
      }

      "typeerrors" => {
        let value = typed(v.get_bool(), "typeerrors", VAL_BOOL)?;
        config.type_errors = *value;
      }

      "globals" => {
        let table = typed(v.get_table(), "globals", VAL_STR_ARRAY)?;

        let mut globals = vec![String::new(); table.size()];

        for (k, v) in table.iter() {
          let ConfigTableKey::F64(key) = k else {
            return Err(ConfigError::GlobalsKeyNotNumeric);
          };

          let slot = (*key as usize)
            .checked_sub(1)
            .and_then(|index| globals.get_mut(index));
          let Some(slot) = slot else {
            return Err(ConfigError::GlobalsInvalidNumericKey);
          };

          let Some(global) = v.get_string() else {
            return Err(ConfigError::GlobalsElementNotString);
          };

          slot.clone_from(global);
        }

        config.globals = globals;
      }

      "aliases" => {
        let aliases = typed(v.get_table(), "aliases", VAL_TABLE)?;

        for (k, v) in aliases.iter() {
          let ConfigTableKey::String(alias_key) = k else {
            return Err(ConfigError::AliasTableKeyNotString);
          };

          let Some(alias_value) = v.get_string() else {
            return Err(ConfigError::AliasTableValueNotString);
          };

          parse_alias(config, alias_key, alias_value, alias_options.as_ref())?;
        }
      }

      _ => {}
    }
  }

  Ok(())
}

/// 对应 C++ `parseLuauConfigTable`：取 "luau" 子表并套用配置。
pub(crate) fn parse_luau_config_table(
  config_table: &ConfigTable,
  config: &mut Config,
  alias_options: Option<AliasOptions>,
) -> Result<(), ConfigError> {
  let Some(luau_value) = config_table.find_str("luau") else {
    return Ok(()); // 无 "luau" 键，无事可做
  };

  let luau_table = typed(luau_value.get_table(), "luau", VAL_TABLE)?;

  create_luau_config_from_luau_table(config, luau_table, alias_options)
}

/// lint 布尔值转 C++ 侧的 "true"/"false" 设置串。
fn lint_setting(enabled: bool) -> &'static str {
  if enabled { SETTING_TRUE } else { SETTING_FALSE }
}

/// lint 表条目（通配符与具名共用）：取布尔值并套用规则。
fn apply_lint(
  config: &mut Config,
  warning_name: &str,
  value: &ConfigValue,
) -> Result<(), ConfigError> {
  let Some(enabled) = value.get_bool() else {
    return Err(ConfigError::LintTableValueNotBoolean);
  };

  parse_lint_rule_string(
    &mut config.enabled_lint,
    &mut config.fatal_lint,
    warning_name,
    lint_setting(*enabled),
    false,
  )
}
