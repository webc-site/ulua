use alloc::format;

use strum::IntoEnumIterator;

use crate::{
  enums::code::Code,
  error::ConfigError,
  functions::bad_setting::{OPT_BOOL, OPT_LINT_COMPAT, SETTING_FALSE, SETTING_TRUE, bad_setting},
  records::{lint_options::LintOptions, lint_warning::LintWarning},
};

fn parse_lint_rule_string_for_code(
  enabled_lints: &mut LintOptions,
  fatal_lints: &mut LintOptions,
  code: Code,
  value: &str,
  compat: bool,
) -> Result<(), ConfigError> {
  match value {
    SETTING_TRUE => enabled_lints.enable_warning(code),
    SETTING_FALSE => enabled_lints.disable_warning(code),
    // compat 模式额外接受旧式 enabled/disabled/fatal
    v if compat => match v {
      "enabled" => {
        enabled_lints.enable_warning(code);
        fatal_lints.disable_warning(code);
      }
      "disabled" => {
        enabled_lints.disable_warning(code);
        fatal_lints.disable_warning(code);
      }
      "fatal" => {
        enabled_lints.enable_warning(code);
        fatal_lints.enable_warning(code);
      }
      _ => return Err(ConfigError::Message(bad_setting(v, OPT_LINT_COMPAT))),
    },
    _ => return Err(ConfigError::Message(bad_setting(value, OPT_BOOL))),
  }

  Ok(())
}

pub(crate) fn parse_lint_rule_string(
  enabled_lints: &mut LintOptions,
  fatal_lints: &mut LintOptions,
  warning_name: &str,
  value: &str,
  compat: bool,
) -> Result<(), ConfigError> {
  // "*" 覆盖全部代码；具名先解析 Code，未知即报错
  let named = (warning_name != "*").then(|| LintWarning::parse_name(warning_name));
  if named == Some(Code::Unknown) {
    return Err(ConfigError::Message(format!("Unknown lint {warning_name}")));
  }

  // 具名单码或 "*" 全表共用同一套派发；strum 迭代序 = 声明序 = 判别值升序，
  // 与旧 `Code::ALL` 数组逐项等价
  let wrap = |err: ConfigError| ConfigError::Message(format!("In key {warning_name}: {err}"));
  let mut apply = |code: Code| {
    parse_lint_rule_string_for_code(enabled_lints, fatal_lints, code, value, compat).map_err(wrap)
  };
  match named {
    Some(code) => apply(code),
    None => Code::iter().try_for_each(apply),
  }
}
