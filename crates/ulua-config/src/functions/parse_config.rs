use alloc::format;

use crate::{
  functions::{
    parse_alias::parse_alias, parse_boolean::parse_boolean, parse_json::parse_json,
    parse_lint_rule_string::parse_lint_rule_string, parse_mode_string::parse_mode_string,
  },
  records::{config::Config, config_options::ConfigOptions},
  type_aliases::error::Error,
};

pub fn parse_config(contents: &str, config: &mut Config, options: &ConfigOptions) -> Error {
  parse_json(contents, |keys, value| {
    match keys {
      [key] if key == "languageMode" => parse_mode_string(&mut config.mode, &value, options.compat),
      [key0, key1] if key0 == "lint" => parse_lint_rule_string(
        &mut config.enabled_lint,
        &mut config.fatal_lint,
        key1,
        &value,
        options.compat,
      ),
      [key] if key == "lintErrors" => parse_boolean(&mut config.lint_errors, &value),
      [key] if key == "typeErrors" => parse_boolean(&mut config.type_errors, &value),
      [key] if key == "globals" => {
        config.globals.push(value);
        None
      }
      [key0, key1] if key0 == "aliases" => {
        parse_alias(config, key1, &value, &options.alias_options)
      }
      // compat 模式兼容旧键 "language"."mode"
      [key0, key1] if options.compat && key0 == "language" && key1 == "mode" => {
        parse_mode_string(&mut config.mode, &value, options.compat)
      }
      _ => Some(format!("Unknown key {}", keys.join("/"))),
    }
  })
}
