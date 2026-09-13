use alloc::string::String;

use ulua_common::records::variant::Variant2;

use crate::{
  functions::{
    parse_alias::parse_alias, parse_lint_rule_string::parse_lint_rule_string,
    parse_mode_string::parse_mode_string,
  },
  records::{
    alias_options::AliasOptions, config::Config, config_table::ConfigTable,
    config_table_key::ConfigTableKey,
  },
};

// 雷同错误串常量化
const ERR_LINT_VALUE_NOT_BOOL: &str = "configuration values in \"lint\" table must be booleans";

/// 对应 C++ `createLuauConfigFromLuauTable`：把 "luau" 子表套用到 `config`。
pub(crate) fn create_luau_config_from_luau_table(
  config: &mut Config,
  luau_table: &ConfigTable,
  alias_options: Option<AliasOptions>,
) -> Option<String> {
  for (k, v) in luau_table.iter() {
    let Some(key) = key_string(k) else {
      return Some(String::from(
        "configuration keys in \"luau\" table must be strings",
      ));
    };

    match key.as_str() {
      "languagemode" => {
        let Some(value) = v.get_string() else {
          return Some(String::from(
            "configuration value for key \"languagemode\" must be a string",
          ));
        };

        if let Some(error_message) = parse_mode_string(&mut config.mode, value, false) {
          return Some(error_message);
        }
      }

      "lint" => {
        let Some(lint) = v.get_table() else {
          return Some(String::from(
            "configuration value for key \"lint\" must be a table",
          ));
        };

        // 先处理通配符，保证覆盖顺序符合预期
        if let Some(value) = lint.find_str("*") {
          let Some(enabled) = value.get_bool() else {
            return Some(String::from(ERR_LINT_VALUE_NOT_BOOL));
          };

          if let Some(error_message) = parse_lint_rule_string(
            &mut config.enabled_lint,
            &mut config.fatal_lint,
            "*",
            lint_setting(*enabled),
            false,
          ) {
            return Some(error_message);
          }
        }

        for (k, v) in lint.iter() {
          let Some(warning_name) = key_string(k) else {
            return Some(String::from(
              "configuration keys in \"lint\" table must be strings",
            ));
          };

          if warning_name == "*" {
            continue; // 上面已处理
          }

          let Some(enabled) = v.get_bool() else {
            return Some(String::from(ERR_LINT_VALUE_NOT_BOOL));
          };

          if let Some(error_message) = parse_lint_rule_string(
            &mut config.enabled_lint,
            &mut config.fatal_lint,
            warning_name,
            lint_setting(*enabled),
            false,
          ) {
            return Some(error_message);
          }
        }
      }

      "linterrors" => {
        let Some(value) = v.get_bool() else {
          return Some(String::from(
            "configuration value for key \"linterrors\" must be a boolean",
          ));
        };

        config.lint_errors = *value;
      }

      "typeerrors" => {
        let Some(value) = v.get_bool() else {
          return Some(String::from(
            "configuration value for key \"typeerrors\" must be a boolean",
          ));
        };

        config.type_errors = *value;
      }

      "globals" => {
        let Some(globals_table) = v.get_table() else {
          return Some(String::from(
            "configuration value for key \"globals\" must be an array of strings",
          ));
        };

        let mut globals = vec![String::new(); globals_table.size()];

        for (k, v) in globals_table.iter() {
          let Some(key) = key_number(k) else {
            return Some(String::from(
              "configuration array \"globals\" must only have numeric keys",
            ));
          };

          let index = *key as usize;
          if index < 1 || globals_table.size() < index {
            return Some(String::from(
              "configuration array \"globals\" contains invalid numeric key",
            ));
          }

          let Some(global) = v.get_string() else {
            return Some(String::from(
              "configuration value in \"globals\" table must be a string",
            ));
          };

          // index 已判定在 1..=size，get_unchecked_mut 免越界检查
          unsafe {
            *globals.get_unchecked_mut(index - 1) = global.clone();
          }
        }

        config.globals = globals;
      }

      "aliases" => {
        let Some(aliases) = v.get_table() else {
          return Some(String::from(
            "configuration value for key \"aliases\" must be a table",
          ));
        };

        for (k, v) in aliases.iter() {
          let Some(alias_key) = key_string(k) else {
            return Some(String::from(
              "configuration keys in \"aliases\" table must be strings",
            ));
          };

          let Some(alias_value) = v.get_string() else {
            return Some(String::from(
              "configuration values in \"aliases\" table must be strings",
            ));
          };

          if let Some(error_message) = parse_alias(config, alias_key, alias_value, &alias_options) {
            return Some(error_message);
          }
        }
      }

      _ => {}
    }
  }

  None
}

/// 对应 C++ `parseLuauConfigTable`：取 "luau" 子表并套用配置。
pub(crate) fn parse_luau_config_table(
  config_table: &ConfigTable,
  config: &mut Config,
  alias_options: Option<AliasOptions>,
) -> Option<String> {
  let Some(luau_value) = config_table.find_str("luau") else {
    return None; // 无 "luau" 键，无事可做
  };

  let Some(luau_table) = luau_value.get_table() else {
    return Some(String::from(
      "configuration value for key \"luau\" must be a table",
    ));
  };

  create_luau_config_from_luau_table(config, luau_table, alias_options)
}

/// lint 布尔值转 C++ 侧的 "true"/"false" 设置串。
fn lint_setting(enabled: bool) -> &'static str {
  if enabled { "true" } else { "false" }
}

fn key_string(key: &ConfigTableKey) -> Option<&String> {
  match &key.0 {
    Variant2::V0(value) => Some(value),
    Variant2::V1(_) => None,
  }
}

fn key_number(key: &ConfigTableKey) -> Option<&f64> {
  match &key.0 {
    Variant2::V0(_) => None,
    Variant2::V1(value) => Some(value),
  }
}
