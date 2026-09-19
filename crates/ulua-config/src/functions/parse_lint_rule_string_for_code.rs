use crate::{
  enums::code::Code,
  functions::bad_setting::{OPT_BOOL, OPT_LINT_COMPAT, SETTING_FALSE, SETTING_TRUE, bad_setting},
  records::lint_options::LintOptions,
};

pub(crate) fn parse_lint_rule_string_for_code(
  enabled_lints: &mut LintOptions,
  fatal_lints: &mut LintOptions,
  code: Code,
  value: &str,
  compat: bool,
) -> Option<String> {
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
      _ => return Some(bad_setting(value, OPT_LINT_COMPAT)),
    },
    _ => return Some(bad_setting(value, OPT_BOOL)),
  }

  None
}
