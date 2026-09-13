use ulua_ast::records::{location::Location, position::Position};
use ulua_config::records::lint_warning::LintWarning;

pub fn check_deprecated_warning(warning: &LintWarning, begin: Position, end: Position, msg: &str) {
  assert_eq!(warning.code, LintWarning::CODE_DEPRECATED_API);
  assert_eq!(warning.location, Location::new(begin, end));
  assert_eq!(warning.text, msg);
}
