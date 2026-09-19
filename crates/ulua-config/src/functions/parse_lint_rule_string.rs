use alloc::format;
use core::slice;

use crate::{
  enums::code::Code,
  functions::parse_lint_rule_string_for_code::parse_lint_rule_string_for_code,
  records::{lint_options::LintOptions, lint_warning::LintWarning},
};

pub fn parse_lint_rule_string(
  enabled_lints: &mut LintOptions,
  fatal_lints: &mut LintOptions,
  warning_name: &str,
  value: &str,
  compat: bool,
) -> Option<String> {
  // "*" 覆盖全部代码；具名先解析 Code，未知即报错
  let named = (warning_name != "*").then(|| LintWarning::parse_name(warning_name));
  if named == Some(Code::Unknown) {
    return Some(format!("Unknown lint {warning_name}"));
  }

  // 具名单码（借自 named，其存续覆盖循环）或 "*" 全表，共用同一循环
  let codes: &[Code] = match &named {
    Some(code) => slice::from_ref(code),
    None => &Code::ALL,
  };

  for &code in codes {
    if let Some(err) =
      parse_lint_rule_string_for_code(enabled_lints, fatal_lints, code, value, compat)
    {
      return Some(format!("In key {warning_name}: {err}"));
    }
  }

  None
}
