use alloc::format;

use crate::{
  enums::code::Code,
  functions::parse_lint_rule_string_for_code::parse_lint_rule_string_for_code,
  records::{lint_options::LintOptions, lint_warning::LintWarning},
  type_aliases::error::Error,
};

pub fn parse_lint_rule_string(
  enabled_lints: &mut LintOptions,
  fatal_lints: &mut LintOptions,
  warning_name: &str,
  value: &str,
  compat: bool,
) -> Error {
  if warning_name == "*" {
    for code in Code::ALL {
      if let Some(err) =
        parse_lint_rule_string_for_code(enabled_lints, fatal_lints, code, value, compat)
      {
        return Some(format!("In key {}: {}", warning_name, err));
      }
    }
  } else {
    let code = LintWarning::parse_name(warning_name);

    if code == Code::Unknown {
      return Some(format!("Unknown lint {}", warning_name));
    }

    if let Some(err) =
      parse_lint_rule_string_for_code(enabled_lints, fatal_lints, code, value, compat)
    {
      return Some(format!("In key {}: {}", warning_name, err));
    }
  }

  None
}
