use alloc::format;

use crate::{enums::code::Code, records::lint_options::LintOptions, type_aliases::error::Error};

pub(crate) fn parse_lint_rule_string_for_code(
  enabled_lints: &mut LintOptions,
  fatal_lints: &mut LintOptions,
  code: Code,
  value: &str,
  compat: bool,
) -> Error {
  if value == "true" {
    enabled_lints.enable_warning(code);
  } else if value == "false" {
    enabled_lints.disable_warning(code);
  } else if compat {
    if value == "enabled" {
      enabled_lints.enable_warning(code);
      fatal_lints.disable_warning(code);
    } else if value == "disabled" {
      enabled_lints.disable_warning(code);
      fatal_lints.disable_warning(code);
    } else if value == "fatal" {
      enabled_lints.enable_warning(code);
      fatal_lints.enable_warning(code);
    } else {
      return Some(format!(
        "Bad setting '{}'.  Valid options are enabled, disabled, and fatal",
        value
      ));
    }
  } else {
    return Some(format!(
      "Bad setting '{}'.  Valid options are true and false",
      value
    ));
  }

  None
}
