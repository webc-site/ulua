use ulua_ast::records::{deprecated_info::DeprecatedInfo, location::Location};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_deprecated_api::LintDeprecatedApi,
};
impl LintDeprecatedApi {
  pub fn report_function_info(
    &mut self,
    location: &Location,
    function_name: &str,
    info: &DeprecatedInfo,
  ) {
    let use_part = info
      .use_suggestion()
      .map(|value| format!(", use '{}' instead", value))
      .unwrap_or_default();
    let reason_part = info
      .reason()
      .map(|value| format!(". {}", value))
      .unwrap_or_default();

    emit_warning(
      unsafe { &mut *self.context },
      Code::DeprecatedApi,
      *location,
      format_args!(
        "Function '{}' is deprecated{}{}",
        function_name, use_part, reason_part
      ),
    );
  }
}
