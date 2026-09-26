use core::fmt::Arguments;

use ulua_ast::records::location::Location;
use ulua_common::functions::format::format;
use ulua_config::{enums::code::Code, records::lint_warning::LintWarning};

use crate::records::lint_context::LintContext;
pub fn emit_warning(
  context: &mut LintContext,
  code: Code,
  location: Location,
  args: Arguments<'_>,
) {
  if !context.warning_enabled(code) {
    return;
  }

  let message = format(args);
  let warning = LintWarning {
    code,
    location,
    text: message,
  };
  context.result.push(warning);
}
